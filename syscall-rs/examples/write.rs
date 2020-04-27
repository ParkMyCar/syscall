use std::io::stdout;

use syscall_rs::io::write;

fn main() -> Result<(), std::io::Error> {
    write(stdout(), "Hello World!")?;
    
    Ok(())
}