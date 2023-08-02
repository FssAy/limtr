use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum Error {
    /// The Limtr was not initialized.
    NotInitialized,
    /// The Limtr was already initialized once.
    AlreadyInitialized,
    /// The Limtr communication was closed unexpectedly.
    LimtrClosed,
    /// Callback sender was closed and will not send any value.
    CallbackCanceled,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self
        )
    }
}

impl std::error::Error for Error {}
