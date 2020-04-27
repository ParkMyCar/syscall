use crate::parse_os_error;

use syscall_macro::syscall;
use syscall_num::*;

pub fn exit<R>(ret_val: R) -> std::io::Result<()> 
where
    R: std::convert::Into<isize>,
{
    parse_os_error!(
        syscall!(EXIT, ret_val.into())
    ).map(|_| ())
}