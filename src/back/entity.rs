use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use crate::back::*;
use crate::{Error, FeatureRaw, Limtr};
use crate::utils::IndexMap;


pub(crate) type Blocks = Arc<Mutex<HashMap<FeatureRaw, Usage>>>;
pub(crate) type IndexMapLimtr = IndexMap::<String, Blocks>;

pub(crate) static LIMTR: OnceCell<Limtr> = OnceCell::new();

pub(crate) fn run(buffer: usize) -> Limtr {
    use tokio::sync::mpsc;

    let (tx, mut rx) = mpsc::channel::<Directive>(buffer);

    let _: JoinHandle<Result<(), Error>> = tokio::spawn(async move {
        let mut index_map = IndexMapLimtr::new();

        while let Some(directive) = rx.recv().await {
            directive.handle(&mut index_map).await;
        }

        Err(Error::LimtrClosed)
    });

    Limtr {
        tx,
    }
}
