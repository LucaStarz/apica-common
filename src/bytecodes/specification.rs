use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Metadata and application specification block fields in Apica bytecodes files.
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum ApicaSpecificationBytecode {
    /// End marker for the specification block.
    EndOfSpecification =    0x00000000,

    /// Application display title.
    Title =                 0x00000001,

    /// Unique identifier string for the application.
    Id =                    0x00000002,

    /// Logger subsystem toggle flag.
    LoggerActivation =      0x00000003,

    /// Initial window width in pixels.
    WindowWidth =           0x00000004,

    /// Initial window height in pixels.
    WindowHeight =          0x00000005,

    /// Application version string.
    Version =               0x00000006,

    /// Maximum created elements (var/const/func/...) at the same time.
    IdCount =               0x00000007,
}