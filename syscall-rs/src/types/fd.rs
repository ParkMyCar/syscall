use std::os::unix::io::{RawFd, AsRawFd};

// In a perfect world we'd have a `AsFileDescriptor` trait instead of a struct, à la
// 
// ```
// pub trait AsFileDescriptor {
//     fn as_fd(&self) -> isize;
// }
//
// impl<T: std::os::unix::io::AsRawFd> AsFileDescriptor for T {...}
// impl AsFileDescriptor for usize {...}
// ```
//
// But unfortunately thats not possible in Rust today. Related error: E0119

/// File descriptors are generally just a signed int, so anything that can be an `isize` can be a file descriptor.
/// 
/// Creating a separate struct allows us to `impl AsRawFd` and satisfy the bounds on syscalls that require a file descriptor.
pub struct FileDescriptor {
    fd: isize,
}

impl AsRawFd for FileDescriptor {
    fn as_raw_fd(&self) -> RawFd {
        self.fd as i32
    }
}

impl From<isize> for FileDescriptor {
    fn from(fd: isize) -> Self {
        FileDescriptor {
            fd
        }
    }
}

impl From<usize> for FileDescriptor {
    fn from(fd: usize) -> Self {
        FileDescriptor {
            fd: fd as isize
        }
    }
}
