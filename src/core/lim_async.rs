use tokio::sync::mpsc;
use crate::back::*;
use crate::{Error, Feature};


/// Main entity used for the communication with the rate limiter.
///
/// There can be only one instance of Limtr in the process lifetime.
/// Initialize it using `Limtr::init` function.
pub struct Limtr {
    pub(crate) tx: mpsc::Sender<Directive>,
}

impl Limtr {
    /// Initializes the Limtr entity.
    ///
    /// Call this function once in the warmup phase of your application.
    ///
    /// Returns Err variant when the initialization failed.
    ///
    /// # Example
    /// ```rust
    /// use limtr::Limtr;
    ///
    /// fn main() {
    ///     if let Err(error) = warmup() {
    ///         println!("Could not complete the warmup phase: {}", error);
    ///         return;
    ///     }
    ///
    ///     // (...)
    /// }
    ///
    /// fn warmup() -> Result<(), Box<dyn std::error::Error>> {
    ///     Limtr::init(16)?;
    /// }
    /// ```
    pub async fn init(buffer: usize) -> Result<(), Error> {
        let limtr = entity::run(buffer);
        entity::LIMTR.set(limtr).map_err(|_| Error::LimtrClosed)
    }

    pub async fn set_limit(id: impl ToString, feature: impl Feature, seconds: u32) -> Result<(), Error> {
        let limtr = Limtr::get()?;

        limtr.tx.send(Directive::SetLimit {
            id: id.to_string(),
            feature: feature.into_feature(),
            seconds,
        }).await.map_err(|_| Error::LimtrClosed)
    }

    pub async fn update_limit(id: impl ToString, feature: impl Feature, seconds: u32, max_calls: usize) -> Result<u64, Error> {
        let limtr = Limtr::get()?;

        let (callback, listener) = tokio::sync::oneshot::channel();

        limtr.tx.send(Directive::UpdateLimit {
            id: id.to_string(),
            feature: feature.into_feature(),
            seconds,
            max_calls,
            callback,
        }).await.map_err(|_| Error::LimtrClosed)?;

        listener.await.map_err(|_| Error::CallbackCanceled)
    }
}

impl Limtr {
    fn get() -> Result<&'static Limtr, Error> {
        entity::LIMTR.get().ok_or_else(|| Error::NotInitialized)
    }
}
