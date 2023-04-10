use std::{
    error::Error,
    io::{self, Write},
};

mod file;
mod term;

pub struct Config {
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let p = std::path::Path::new(&config.file_path);
    let mut state = file::get_initial_state(p).expect("cannot get state for path");

    loop {
        term::move_cursor_to_top();
        turn(&mut state);
        term::print_state(&state);
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100))
    }
}

fn turn(state: &mut Vec<Vec<bool>>) {
    let rows = state.len() as i32;
    let cols = state[0].len() as i32;
    let mut copy = vec![vec![false; cols as usize]; rows as usize];

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
                if v {
                    alive_n_count += 1;
                }
            }
            if cell {
                if alive_n_count < 2 {
                    copy[m as usize][n as usize] = false;
                } else if alive_n_count > 3 {
                    copy[m as usize][n as usize] = false;
                } else {
                    copy[m as usize][n as usize] = true;
                }
            } else {
                if alive_n_count == 3 {
                    copy[m as usize][n as usize] = true;
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