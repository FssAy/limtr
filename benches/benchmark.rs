use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;
use limtr::*;

const FEATURE_ANY: FeatureRaw = FeatureRaw::MAX;

macro_rules! limtr {
    () => {
        LIMTR.get().expect("Limtr was not initialized!")
    };
}

macro_rules! bench {
    (
        $(
            fn $func:ident (
                buffer:$buffer:literal
            )
            $body:block
        )*
    ) => {
        $(
            fn $func(c: &mut Criterion) {
                static LIMTR: OnceCell<Limtr> = OnceCell::new();
                let runtime = Runtime::new().unwrap();

                runtime.block_on(async move {
                    LIMTR.get_or_init(|| Limtr::new($buffer));
                });

                c.bench_function(stringify!($func), |b| {
                    b.to_async(&runtime).iter(|| async move {
                        $body
                    });
                });
            }
        )*
    };
}

bench!(
    fn limit_update(buffer: 1024) {
        limtr!().update_limit_local(
            "user-1",
            FEATURE_ANY,
            20,
            10_000,
        ).await.unwrap();
    }

    fn limit_update_filled(buffer: 1) {
        limtr!().update_limit_local(
            "user-2",
            FEATURE_ANY,
            20,
            10_000,
        ).await.unwrap();
    }

    fn limit_update_blocked(buffer: 1024) {
        limtr!().update_limit_local(
            "user-3",
            FEATURE_ANY,
            60*10,
            1,
        ).await.unwrap();
    }

    fn limit_update_expired(buffer: 1024) {
        limtr!().update_limit_local(
            "user-4",
            FEATURE_ANY,
            1,
            3,
        ).await.unwrap();
    }
);

criterion_group!(
    benches,
    limit_update,
    limit_update_filled,
    limit_update_blocked,
    limit_update_expired,
);

criterion_main!(benches);
