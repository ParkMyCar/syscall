use std::path::Path;

use syscall_rs::{types::fd::FileDescriptor, file::{close, open}, io::read};

fn main() -> Result<(), std::io::Error> {
    // Make a path to our favorite file
    let path = Path::new("examples/files/hello_world.txt");

    // Open it, and get a file descriptor
    let fd = open(&path, 0, 0)?;
    
    // Create our buffer to write into
    let mut buf = vec![0_u8; 16];
    // lil-boilerplate to satisfy trait bounds
    let fd: FileDescriptor = fd.into();

    // Read an arbitrary amount of bytes into our buffer!
    read(&fd, &mut buf, 16)?;
    let string = String::from_utf8_lossy(&buf);
    println!("{}", string);

    // You should be courteous and close file descriptors when you're done!
    close(fd)?;

    Ok(())
}
