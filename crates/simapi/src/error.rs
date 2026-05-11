use thiserror::Error;

/// Errors returned by the safe `simapi` wrapper.
#[derive(Debug, Error)]
pub enum Error {
    /// The crate was used on an unsupported target platform.
    #[error("simapi is only supported on Linux")]
    UnsupportedPlatform,

    /// The upstream library failed to allocate a `SimMap`.
    #[error("failed to create SimMap")]
    CreateMap,

    /// Reserved for failures when mapping shared memory directly.
    #[error("failed to map simapi shared memory, error code {0}")]
    OpenMap(i32),

    /// The upstream library returned an error while refreshing telemetry.
    #[error("failed to read telemetry, error code {0}")]
    ReadTelemetry(i32),

    /// No active simapi producer could be found.
    #[error("simd is not running or the simapi consumer map is unavailable")]
    SimdUnavailable,

    /// The snapshot was produced by a different simapi version.
    #[error("simapi version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: u8, actual: u8 },

    /// Reserved for string conversion failures.
    #[error("invalid UTF-8 or C string conversion")]
    InvalidString,
}

/// Result type used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;
