#![allow(dead_code)]

use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;
use crate::{Error, Feature, FeatureRaw, Limtr};


#[repr(u16)]
#[derive(Copy, Clone, Debug)]
enum F {
    A,
    B,
    C,
}

impl Feature for F {
    fn into_feature(self) -> FeatureRaw {
        self as FeatureRaw
    }
}

async fn validate_update<F: Future<Output=Result<u64, Error>>>(future: F, blocked: bool) {
    let exp = future.await.unwrap();
    if blocked {
        assert_ne!(exp, 0);
    } else {
        assert_eq!(exp, 0);
    }
}

#[tokio::test]
async fn single_call() {
    let limtr = Limtr::new(16);

    const ID: &'static str = "single_call";
    const FEATURE: F = F::A;
    const SECONDS: u32 = 10;
    const MAX_CALLS: usize = 1;

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        false,
    ).await;

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        true,
    ).await;
}

#[tokio::test]
async fn multiple_calls() {
    let limtr = Limtr::new(16);

    const ID: &'static str = "multiple_calls";
    const FEATURE: F = F::A;
    const SECONDS: u32 = 10;
    const MAX_CALLS: usize = 3;

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        false,
    ).await;

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        false,
    ).await;

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        false,
    ).await;

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        true,
    ).await;
}

#[tokio::test]
async fn clear_calls() {
    let limtr = Limtr::new(16);

    const ID: &'static str = "clear_calls";
    const FEATURE: F = F::A;
    const SECONDS: u32 = 1;
    const MAX_CALLS: usize = 2;

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        false,
    ).await;

    sleep(Duration::from_millis(1500)).await;

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        false,
    ).await;

    sleep(Duration::from_millis(1500)).await;

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        false,
    ).await;
}

#[tokio::test]
async fn block_check() {
    let limtr = Limtr::new(16);

    const ID: &'static str = "block_check";
    const FEATURE: F = F::A;
    const SECONDS: u32 = 10;
    const MAX_CALLS: usize = 2;

    assert_eq!(limtr.get_limit_local(ID, FEATURE).await.unwrap(), 0);

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        false,
    ).await;

    assert_eq!(limtr.get_limit_local(ID, FEATURE).await.unwrap(), 0);

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        false,
    ).await;

    validate_update(
        limtr.update_limit_local(ID, FEATURE, SECONDS, MAX_CALLS),
        true,
    ).await;

    assert_ne!(limtr.get_limit_local(ID, FEATURE).await.unwrap(), 0);
}
