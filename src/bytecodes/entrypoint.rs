use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Lifecycle entrypoint opcodes for an Apica application execution.
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum ApicaEntrypointBytecode {
    /// Initialization hook (runs once on startup).
    Init =              0x00000000,

    /// Frame update hook (runs every frame loop).
    Update =            0x00000001,

    /// Teardown hook (runs upon exit).
    Quit =              0x00000002,
}