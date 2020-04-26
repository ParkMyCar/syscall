pub use platform::*;

#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
pub mod platform;
