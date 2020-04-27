use std::fs::File;

use bytes::BytesMut;
use syscall_rs::io::read;

fn main() -> Result<(), std::io::Error> {
    let mut buf = BytesMut::with_capacity(16);
    buf.resize(16, 0); // with_capacity(...) does not allocate

    let file = File::open("examples/files/hello_world.txt")?;
    
    read(file, &mut buf, 16)?;
    println!("{:#?}", buf);

    Ok(())
}