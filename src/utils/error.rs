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
