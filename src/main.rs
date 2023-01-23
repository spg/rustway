use std::io::{self, Write};

mod file;
mod term;

fn print_state(state: &Vec<Vec<i32>>) {
    for row in state {
        for cell in row {
            // println!("{}", cell.clone());
            let c = if cell.clone() > 0 { "*" } else { " " };
            print!("{}", c);
        }
    }
}

fn turn(state: &mut Vec<Vec<i32>>) {
    let rows = state.len() as i32;
    let cols = state[0].len() as i32;
    let mut copy = vec![vec![0; cols as usize]; rows as usize];

    for m in 0..rows - 1 {
        for n in 0..cols - 1 {
            let cell = state[m as usize][n as usize];
            let mut n_coords: Vec<Vec<i32>> = vec![];

            n_coords.push(vec![m - 1, n + 1]);
            n_coords.push(vec![m, n + 1]);
            n_coords.push(vec![m + 1, n + 1]);
            n_coords.push(vec![m + 1, n]);
            n_coords.push(vec![m + 1, n - 1]);
            n_coords.push(vec![m, n - 1]);
            n_coords.push(vec![m - 1, n - 1]);
            n_coords.push(vec![m - 1, n]);

            let valid_coords: Vec<Vec<i32>> = n_coords
                .iter()
                .filter(|c| c[0] >= 0 && c[0] < rows && c[1] >= 0 && c[1] < cols)
                .cloned()
                .collect();

            let mut alive_n_count = 0;
            for v_coord in valid_coords {
                let m = v_coord[0];
                let n = v_coord[1];
                let v = state[m as usize][n as usize];
                if v > 0 {
                    alive_n_count += 1;
                }
            }
            if cell > 0 {
                if alive_n_count < 2 {
                    copy[m as usize][n as usize] = 0;
                } else if alive_n_count > 3 {
                    copy[m as usize][n as usize] = 0;
                } else {
                    copy[m as usize][n as usize] = 1;
                }
            } else {
                if alive_n_count == 3 {
                    copy[m as usize][n as usize] = 1;
                }
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            state[i as usize][j as usize] = copy[i as usize][j as usize]
        }
    }
}

fn main() {
    let w = unsafe { term::get_dimensions() };
    let w_cells = w.ws_col as i32;
    let h_cells = w.ws_row as i32;

    let mut state = vec![vec![0; w_cells as usize]; h_cells as usize];

    state[0][1] = 1;
    state[1][3] = 1;
    state[1][4] = 1;
    state[2][3] = 1;
    state[2][4] = 1;
    state[3][3] = 1;

    // beacon
    state[20][20] = 1;
    state[20][21] = 1;
    state[21][20] = 1;
    state[21][21] = 1;
    state[22][22] = 1;
    state[22][23] = 1;
    state[23][22] = 1;
    state[23][23] = 1;

    // blinker
    state[20][40] = 1;
    state[21][40] = 1;
    state[22][40] = 1;

    // glider
    state[50][70] = 1;
    state[51][71] = 1;
    state[51][72] = 1;
    state[50][72] = 1;
    state[49][72] = 1;

    loop {
        term::move_cursor_to_top();
        turn(&mut state);
        print_state(&state);
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100))
    }
}
