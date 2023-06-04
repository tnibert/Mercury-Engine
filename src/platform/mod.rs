/**
 * This module isolates platform specific code.
 */

#[cfg(target_family = "wasm")]
pub mod wasmplatform;
#[cfg(target_family = "unix")]
pub mod desktopplatform;