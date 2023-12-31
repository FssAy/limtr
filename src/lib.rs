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
    mod feature;
    pub(crate) mod time;

    pub use feature::*;
}

pub use crate::core::*;
pub use utils::{error::Error, Feature, FeatureRaw};
