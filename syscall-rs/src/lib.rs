use syscall_macro::syscall;

pub use platform::*;

#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
pub mod platform;

pub mod macros;
