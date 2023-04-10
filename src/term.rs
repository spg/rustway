use libc::{c_int, c_ulong, winsize, STDOUT_FILENO};
use std::mem::zeroed;

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...);
}

const TIOCGWINSZ: c_ulong = 0x40087468;

pub fn get_dimensions() -> winsize {
    let window: winsize;
    unsafe {
        window = zeroed();
        ioctl(STDOUT_FILENO, TIOCGWINSZ, &window)
    };
    window
}

pub fn move_cursor_to_top() {
    print!("\x1B[1;1H");
    print!("{}[2J", 27 as char); // clear screen
}

pub fn print_state(state: &Vec<Vec<bool>>) {
    for row in state {
        for cell in row {
            let c = if cell.clone() == true { "*" } else { " " };
            print!("{}", c);
        }
        print!("\n");
    }
}
