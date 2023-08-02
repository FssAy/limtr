use tokio::sync::mpsc;
use crate::back::*;
use crate::Error;


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
}