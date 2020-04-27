use std::path::Path;

use syscall_rs::{types::fd::FileDescriptor, file::open, io::read};

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("examples/files/hello_world.txt");
    let fd = open(&path, 0, 0)?;
    
    let mut buf = vec![0_u8; 16];
    let fd: FileDescriptor = fd.into();

    read(fd, &mut buf, 16)?;
    let string = String::from_utf8_lossy(&buf);
    println!("{}", string);

    Ok(())
}
