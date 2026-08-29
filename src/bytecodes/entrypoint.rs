use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Lifecycle entrypoint opcodes for an Apica application execution.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
pub enum ApicaEntrypointBytecode {
    /// Initialization hook (runs once on startup).
    Init =              0x00,

    /// Frame update hook (runs every frame loop).
    Update =            0x01,

    /// Teardown hook (runs upon exit).
    Quit =              0x02,
}