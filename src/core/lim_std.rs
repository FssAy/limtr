use std::sync::{Arc, mpsc};
use std::thread::JoinHandle;
use crate::back::*;
use crate::Error;


/// Main entity used for the communication with the rate limiter.
///
/// There can be only one instance of Limtr in the process lifetime.
/// Initialize it using `Limtr::init` function.
pub struct Limtr {
    pub(crate) tx: mpsc::Sender<Directive>,
    pub(crate) handle: Arc<JoinHandle<Result<(), Error>>>,
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
    ///     if Limtr::check() {
    ///         println!("Limtr is running!")
    ///     }
    /// }
    ///
    /// fn warmup() -> Result<(), Box<dyn std::error::Error>> {
    ///     Limtr::init()?;
    /// }
    /// ```
    pub fn init() -> Result<(), Error> {
        let limtr = entity::run();
        entity::LIMTR.set(limtr).map_err(|_| Error::LimtrClosed)
    }

    //// Checks if the rate limiter is running.
    // pub fn check() -> bool {
    //     if let Some(limtr) = entity::LIMTR.get() {
    //         !limtr.handle.is_finished()
    //     } else {
    //         false
    //     }
    // }
}