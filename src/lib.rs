#![no_std]
#![feature(lang_items)]
//extern crate libc;

#[lang="panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
        loop {}
}

extern {
    fn color_screen();
}

#[no_mangle]
pub extern "C" fn rust_call()
{
    unsafe { color_screen(); }
}
