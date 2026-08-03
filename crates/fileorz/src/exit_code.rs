//! CLI exit codes for the `fileorz` binary.

/// Success.
pub const OK: u8 = 0;
/// Generic runtime / organize failure.
pub const ERROR: u8 = 1;
/// Config load / parse / migrate failure.
pub const CONFIG: u8 = 2;
/// Organize folder missing, not a directory, or not writable.
pub const FOLDER: u8 = 3;
/// Bad CLI usage / missing required flags.
pub const USAGE: u8 = 4;
