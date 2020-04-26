#![cfg_attr(feature = "asm", feature(llvm_asm))]

#[doc(hidden)]
#[cfg(not(feature = "asm"))]
#[path = "syscall/c.rs"]
#[macro_use]
pub mod syscall;

#[doc(hidden)]
#[cfg(feature = "asm")]
#[path = "syscall/asm.rs"]
#[macro_use]
pub mod syscall;

#[macro_export]
macro_rules! syscall {
    ($call:expr) => {
        unsafe {
            $crate::__syscall!($call)
        }
    };

    ($call:expr, $($arg:expr),*) => {
        unsafe {
            $crate::__syscall!($call, $($arg),+)
        }
    };
}
