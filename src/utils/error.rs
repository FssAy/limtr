pub enum Error {
    /// The Limtr was already initialized once.
    AlreadyInitialized,
    /// The Limtr communication was closed unexpectedly.
    LimtrClosed,
}
