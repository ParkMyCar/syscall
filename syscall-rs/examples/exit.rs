use syscall_rs::exit;

fn main() -> Result<(), std::io::Error> {
    println!("This will get printed");
    exit(0_u8)?;
    println!("But this won't!");

    Ok(())
}