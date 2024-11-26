#![no_std]
use core::arch::asm;

pub use dos_to_go_procmacro::entrypoint as entrypoint;

pub fn exit(code: u8) {
    unsafe {
        asm!(
            "mov ah, 0x4C",
            "int 0x21",
            in("al") code
        );
    }
}

pub fn putc(c: char) {
    unsafe {
        asm!(
            "mov ah, 0x02",
            "int 0x21",
            in("dl") c as u8
        );
    }
}

pub fn puts(s: &str) {
    unsafe {
        asm!(
            "mov ah, 0x09",
            "int 0x21",
            in("edx") s.as_ptr()
        );
    }
}
