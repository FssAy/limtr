#[cfg(test)]
mod tests;

mod core {
    // todo: sync version

    #[cfg(feature = "async")]
    mod lim_async;
    #[cfg(feature = "async")]
    pub use lim_async::*;
}

mod back {
    mod com;
    pub(crate) mod entity;

    pub(crate) use com::*;
}

mod utils {
    pub mod error;
    mod index_map;
    pub(crate) mod time;

    pub(crate) use index_map::*;

    pub type FeatureRaw = u16;

    pub trait Feature {
        fn into_feature(self) -> FeatureRaw;
    }
}

pub use crate::core::*;
pub use utils::{error::Error, Feature, FeatureRaw};
pub(crate) use utils::*;
pub(crate) use back::*;
