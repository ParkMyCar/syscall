use syscall_macro::syscall;

pub use platform::*;

#[cfg(all(target_os="macos", target_arch="x86_64"))]
#[path = "platform/macos-x86_64/mod.rs"]
pub mod platform;

pub mod macros;
pub mod types;
