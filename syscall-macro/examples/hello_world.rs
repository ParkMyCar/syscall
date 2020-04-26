use syscall_macro::syscall;
use syscall_num::{EXIT, GETGID, WRITE};

fn main() {
    let text = "Hello World!\n".to_owned();
    let bytes = text.as_bytes();

    syscall!(WRITE, 1, bytes.as_ptr(), bytes.len()); // write

    let gid = syscall!(GETGID); // getgid
    println!("gid: {}", gid);

    syscall!(EXIT, 0); // exit

    println!("This shouldn't get printed!");
}
