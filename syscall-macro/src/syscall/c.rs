#[cfg(feature = "default")]
use libc;

extern "C" {
    pub fn syscall(_: libc::c_int, ...) -> libc::c_int;
}

#[doc(hidden)]
#[macro_export]
macro_rules! __syscall {
    ($call:expr) => {
        unsafe {
            syscall_macro::syscall::syscall($call)
        }
    };

    ($call:expr, $($arg:expr),+) => {
        unsafe {
            syscall_macro::syscall::syscall($call, $($arg),+)
        }
    };
}
