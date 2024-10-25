// src/main.rs
#![no_std]
#![no_main]

use core::ffi::c_void;
use core::fmt;
use core::result::Result;
use core::result::Result::Ok;

extern "C" {
    fn write(fildes: i32, buf: *const c_void, nbyte: usize);
}

pub fn write_string(s: &str) {
    unsafe {
        write(1, s.as_ptr() as *const c_void, s.len());
    }
}

pub struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        write_string(s);
        Ok(())
    }
}

#[allow(unused)]
pub fn print(args: fmt::Arguments) {
    use fmt::Write;
    Writer {}.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        print(format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! println {
    () => {{
        print!("\n");
    }};

    ($($arg:tt)*) => {{
        print!("{}\n", format_args!($($arg)*));
    }}
}

extern "C" {
    fn exit(status: i32) -> !;
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    unsafe {
        exit(1);
    }
}

#[no_mangle]
pub extern "C" fn main() -> u32 {
    panic!("Somebody set us the bomb");
    0
}
