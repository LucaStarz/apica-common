use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Opcodes for native functions provided out-of-the-box by the Apica system.
///
/// Includes core utilities for application management, console logging,
/// user input polling, 2D sprite rendering and more.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
pub enum ApicaBuiltinFunctionBytecode {
    /// Terminate application execution immediately.
    QuitApp =                   0x00000000,

    /// Log an informational message to the `logs` folder.
    LogInfo =                   0x00000001,

    /// Log an informational message to the `logs` folder with a trailing newline.
    LognInfo =                  0x00000002,

    /// Log a success message to the `logs` folder.
    LogSuccess =                0x00000003,

    /// Log a success message to the `logs` folder with a trailing newline.
    LognSuccess =               0x00000004,

    /// Log a warning message to the `logs` folder.
    LogWarning =                0x00000005,

    /// Log a warning message to the `logs` folder with a trailing newline.
    LognWarning =               0x00000006,

    /// Log an error message to the `logs` folder.
    LogError =                  0x00000007,

    /// Log an error message to the `logs` folder with a trailing newline.
    LognError =                 0x00000008,

    /// Load and switch to a new application.
    LoadApp =                   0x00000009,

    /// Set window title bar text.
    SetTitle =                  0x0000000A,

    /// Enable or disable window resizability.
    SetResizable =              0x0000000B,

    /// Check if a key is currently in a released state.
    IsKeyReleased =             0x0000000C,

    /// Check if a key was pressed down in the current frame.
    IsKeyJustPressed =          0x0000000D,

    /// Check if a key is currently in a pressed state.
    IsKeyPressed =              0x0000000E,

    /// Check if a key was released in the current frame.
    IsKeyJustReleased =         0x0000000F,

    /// Load a 2D spritesheet into GPU memory.
    LoadSpritesheet2D =         0x00000010,

    /// Unload a 2D spritesheet from GPU memory.
    UnloadSpritesheet2D =       0x00000011,

    /// Render a specific sprite at a given 2D coordinates.
    DrawSprite2DAt =            0x00000012,

    /// Get the delta-time of the VM.
    GetDeltaTime =              0x00000013,

    /// Get the number of columns of a 2D spritesheet.
    GetSpritesheet2DColumns =   0x00000014,

    /// Get the number of rows of a 2D spritesheet.
    GetSpritesheet2DRows =      0x00000015,

    /// Get the number of frames of a 2D spritesheet.
    GetSpritesheet2DFrames =    0x00000016,

    /// Set the Clear Color of the renderer.
    SetClearColor =             0x00000017,

    /// Set the window size.
    SetWindowSize =             0x00000018,

    /// Set the language of the system.
    SetSystemLanguage =         0x00000019,

    /// Get the language of the system.
    GetSystemLanguage =         0x0000001A,

    /// Get a string representing a specific date.
    GetFormattedDate =          0x0000001B,

    /// Get a string representing the actual date.
    GetActualFormattedDate =    0x0000001C,

    /// Get a string representing a specific time.
    GetFormattedTime =          0x0000001D,

    /// Get a string representing the actual time.
    GetActualFormattedTime =    0x0000001E,
    
    /// Display a text at a given position.
    DisplayTextAt =             0x0000001F,
}