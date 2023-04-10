pub fn move_cursor_to_top() {
    print!("\x1B[1;1H");
    print!("{}[2J", 27 as char);
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
