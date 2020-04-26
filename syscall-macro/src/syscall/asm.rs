#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const OFFSET: usize = 0x2000000;

#[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
const OFFSET: usize = 0;

#[doc(hidden)]
#[inline(always)]
pub unsafe fn syscall0(n: usize) -> isize {
    let ret: isize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n + OFFSET)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[doc(hidden)]
#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> isize {
    let ret: isize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n + OFFSET), "{rdi}"(a1)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[doc(hidden)]
#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> isize {
    let ret: isize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n + OFFSET), "{rdi}"(a1), "{rsi}"(a2)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[doc(hidden)]
#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> isize {
    let ret: isize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n + OFFSET), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[doc(hidden)]
#[inline(always)]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> isize {
    let ret: isize;
    llvm_asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n + OFFSET), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[doc(hidden)]
#[macro_export]
macro_rules! __syscall {
    ($call:expr) => {
        crate::syscall::syscall0($call)
    };
    ($call:expr, $arg1:expr) => {
        crate::syscall::syscall1($call, $arg1 as usize)
    };
    ($call:expr, $arg1:expr, $arg2:expr) => {
        crate::syscall::syscall2($call, $arg1 as usize, $arg2 as usize)
    };
    ($call:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        crate::syscall::syscall3($call, $arg1 as usize, $arg2 as usize, $arg3 as usize)
    };
    ($call:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        crate::syscall::syscall4(
            $call,
            $arg1 as usize,
            $arg2 as usize,
            $arg3 as usize,
            $arg4 as usize,
        )
    };
}
