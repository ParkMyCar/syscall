use crate::parse_os_error;

use std::convert::{AsMut, AsRef};
use std::os::unix::io::AsRawFd;

use syscall_macro::syscall;
use syscall_num::*;

pub fn read<F, B>(fd: F, mut buf: B, num: usize) -> std::io::Result<usize>
where 
    F: AsRawFd,
    B: AsMut<[u8]>,
{
    let raw_fd = fd.as_raw_fd();
    let buf_ref = buf.as_mut();

    parse_os_error!(
        syscall!(READ, raw_fd, buf_ref.as_mut_ptr(), num)
    )
}

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