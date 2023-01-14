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

fn clear_term() {
    print!("\x1B[2J")
}

fn move_cursor_to_top() {
    print!("\x1B[1;1H")
}

fn draw_grid() {
    print!("*****")
}

fn main() {
    let w = unsafe { get_dimensions() };

    loop {
        clear_term();
        move_cursor_to_top();
        draw_grid()
    }
}
