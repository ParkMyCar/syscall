extern "C" {
    pub fn syscall(_: isize, ...) -> usize;
}

#[doc(hidden)]
#[macro_export]
macro_rules! __syscall {
    ($call:expr) => {
        unsafe {
            crate::syscall::syscall($call)
        }
    };

    ($call:expr, $($arg:expr),+) => {
        unsafe {
            crate::syscall::syscall($call, $($arg),+)
        }
    };
}
