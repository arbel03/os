// Copyright (c) 2017 Redox OS Developers

// MIT License

// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:

// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
#![allow(dead_code)]

pub unsafe fn syscall0(mut a: usize) -> usize {
    asm!("int 0x80"
        : "={eax}"(a)
        : "{eax}"(a)
        : "memory"
        : "intel", "volatile");

    a
}

pub unsafe fn syscall1(mut a: usize, b: usize) -> usize {
    asm!("int 0x80"
        : "={eax}"(a)
        : "{eax}"(a), "{ebx}"(b)
        : "memory"
        : "intel", "volatile");

    a
}

// Clobbers all registers - special for clone
pub unsafe fn syscall1_clobber(mut a: usize, b: usize) -> usize {
    asm!("int 0x80"
        : "={eax}"(a)
        : "{eax}"(a), "{ebx}"(b)
        : "memory", "ebx", "ecx", "edx", "esi", "edi"
        : "intel", "volatile");

    a
}

pub unsafe fn syscall2(mut a: usize, b: usize, c: usize) -> usize {
    asm!("int 0x80"
        : "={eax}"(a)
        : "{eax}"(a), "{ebx}"(b), "{ecx}"(c)
        : "memory"
        : "intel", "volatile");

    a
}

pub unsafe fn syscall3(mut a: usize, b: usize, c: usize, d: usize) -> usize {
    asm!("int 0x80"
        : "={eax}"(a)
        : "{eax}"(a), "{ebx}"(b), "{ecx}"(c), "{edx}"(d)
        : "memory"
        : "intel", "volatile");

    a
}

pub unsafe fn syscall4(mut a: usize, b: usize, c: usize, d: usize, e: usize) -> usize {
    asm!("int 0x80"
        : "={eax}"(a)
        : "{eax}"(a), "{ebx}"(b), "{ecx}"(c), "{edx}"(d), "{esi}"(e)
        : "memory"
        : "intel", "volatile");

    a
}

pub unsafe fn syscall5(mut a: usize, b: usize, c: usize, d: usize, e: usize, f: usize) -> usize {
    asm!("int 0x80"
        : "={eax}"(a)
        : "{eax}"(a), "{ebx}"(b), "{ecx}"(c), "{edx}"(d), "{esi}"(e), "{edi}"(f)
        : "memory"
        : "intel", "volatile");
    a
}