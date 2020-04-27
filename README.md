# syscall [WIP]

A Rust interface for *nix OS system calls. 

Provided from this workspace are multiple crates that build on eachother, the one you want is probably `syscall-rs`.
<br />
<br />

## Crate Overview 📦
* `syscall-rs` - A Rust interface of (soon to be) all C syscalls, e.g. `fn open(path: Path, flags: usize, mode: usize)`
* `syscall-num` - Constant values of all syscalls
* `syscall-macro` - Provides only a `syscall!(...)` macro that allows you to make syscalls from Rust
