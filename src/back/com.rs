use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};
use crate::back::entity::{Blocks, IndexMapLimtr};
use crate::FeatureRaw;
use crate::utils::time;


pub(crate) type Callback<T> = oneshot::Sender<T>;

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Usage {
    calls: usize,
    expiration: u64,
    last_call: u64,
}

#[derive(Debug)]
pub(crate) enum Directive {
    SetLimit {
        id: String,
        feature: FeatureRaw,
        seconds: u32,
    },
    UpdateLimit {
        id: String,
        feature: FeatureRaw,
        max_calls: usize,
        seconds: u32,
        callback: Callback<u64>,
    },
    GetLimit {
        id: String,
        feature: FeatureRaw,
        callback: Callback<u64>,
    },
    Clear,
}

impl Directive {
    pub async fn handle(self, index_map: &mut IndexMapLimtr) { match self {
        Directive::SetLimit { id, feature, seconds } => {
            let blocks = get_or_create_blocks(index_map, id);
            tokio::spawn(async move {
                let calls = 0;
                let expiration = time::in_future(seconds);

                let mut lock = blocks.lock().await;

                if let Some(usage) = lock.get_mut(&feature) {
                    usage.calls = calls;
                    usage.expiration = expiration;
                    usage.last_call = time::now();
                } else {
                    lock.insert(feature, Usage {
                        calls,
                        expiration,
                        last_call: time::now(),
                    });
                }

                drop(lock);
            });
        },
        Directive::UpdateLimit { id, feature, max_calls, seconds, callback } => {
            let blocks = get_or_create_blocks(index_map, id);
            tokio::spawn(async move {
                let mut lock = blocks.lock().await;

                let mut exp = 0;
                let now = time::now();

                if let Some(usage) = lock.get_mut(&feature) {
                    if usage.calls >= max_calls {
                        usage.calls = 0;
                        usage.expiration = time::from_point(now, seconds);
                        usage.last_call = now;
                    } else if usage.expiration <= now && usage.expiration != 0 {
                        usage.calls = 1;
                        usage.expiration = 0;
                        usage.last_call = now;
                    } else if time::from_point(usage.last_call, seconds) <= now && usage.expiration == 0 {
                        usage.calls = 0;
                        usage.last_call = now;
                    } else {
                        if usage.expiration == 0 {
                            usage.calls += 1;
                            usage.last_call = now;
                        }
                    }

                    exp = usage.expiration;
                } else {
                    lock.insert(feature, Usage {
                        calls: 1,
                        expiration: exp,
                        last_call: now,
                    });
                }

                drop(lock);

                let _ = callback.send(exp);
            });
        },
        Directive::GetLimit { id, feature, callback } => {
            if let Some(blocks) = index_map.get(&id).map(Arc::clone) {
                tokio::spawn(async move {
                    let lock = blocks.lock().await;
                    let mut exp = 0;

                    if let Some(usage) = lock.get(&feature) {
                        if time::now() < usage.expiration {
                            exp = usage.expiration;
                        }
                    }

                    let _ = callback.send(exp);
                });
            } else {
                let _ = callback.send(0);
            }
        }
        Directive::Clear => {
            index_map.clear_all();
        }
    } }
}

unsafe impl Send for Directive {}
unsafe impl Sync for Directive {}

fn get_or_create_blocks(index_map: &mut IndexMapLimtr, id: String) -> Blocks {
    if let Some(blocks) = index_map.get(&id) {
        blocks.clone()
    } else {
        let blocks = Arc::new(
            Mutex::new(
                HashMap::<FeatureRaw, Usage>::new()
            )
        );

        index_map.insert(id, blocks.clone());
        blocks
    }
}
