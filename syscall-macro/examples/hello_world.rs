use syscall_macro::syscall;

fn main() {
    let text = "Hello World!\n".to_owned();
    let bytes = text.as_bytes();

    syscall!(4, 1, bytes.as_ptr(), bytes.len()); // write

    let gid = syscall!(43); // getgid
    println!("gid: {}", gid);

    syscall!(1, 0); // exit

    println!("This shouldn't get printed!");
}
