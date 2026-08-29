use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Metadata and application specification block fields in Apica bytecodes files.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum ApicaSpecificationBytecode {
    /// End marker for the specification block.
    EndOfSpecification =    0x0000,

    /// Application display title.
    Title =                 0x0001,

    /// Unique identifier string for the application.
    Id =                    0x0002,

    /// Logger subsystem toggle flag.
    LoggerActivation =      0x0003,

    /// Initial window width in pixels.
    WindowWidth =           0x0004,

    /// Initial window height in pixels.
    WindowHeight =          0x0005,

    /// Application version string.
    Version =               0x0006,

    /// Maximum created elements (var/const/func/...) at the same time.
    IdCount =               0x0007,
}