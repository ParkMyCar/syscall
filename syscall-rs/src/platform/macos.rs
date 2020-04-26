use crate::parse_os_error;

use std::convert::AsRef;
use std::os::unix::io::AsRawFd;

use syscall_macro::syscall;
use syscall_num::*;

pub fn write<F, B>(fd: F, bytes: B) -> std::io::Result<usize>
where
    F: AsRawFd,
    B: AsRef<[u8]>,
{
    let raw_fd = fd.as_raw_fd();
    let bytes_ref = bytes.as_ref();

    parse_os_error!(
        syscall!(WRITE, raw_fd, bytes_ref.as_ptr(), bytes_ref.len())
    )   
}
