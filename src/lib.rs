#![no_std]
#![feature(lang_items)]

#[lang="panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
        loop {}
}
