use libc::{c_int, c_ulong, winsize, STDOUT_FILENO};
use std::mem::zeroed;

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...);
}

const TIOCGWINSZ: c_ulong = 0x40087468;

unsafe fn get_dimensions() -> winsize {
    let window: winsize = zeroed();
    unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &window) };
    window
}

fn main() {
    let w = unsafe { get_dimensions() };
    println!("{}", w.ws_col);
    println!("{}", w.ws_row);
}
