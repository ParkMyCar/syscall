use crate::parse_os_error;

use std::convert::AsRef;
use std::ffi::{CString, OsStr};
use std::os::unix::ffi::OsStrExt;

use syscall_macro::syscall;
use syscall_num::*;

pub fn open<P>(path: P, flags: usize, mode: usize) -> std::io::Result<usize> 
where
    P: AsRef<OsStr>
{
    let path_bytes = path.as_ref().as_bytes();
    let c_string = CString::new(path_bytes)?;

    parse_os_error!(
        syscall!(OPEN, c_string.as_bytes().as_ptr(), flags, mode) 
    )
}