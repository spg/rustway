use std::{error::Error, fs};

#[cfg(test)]
mod tests {
    use crate::file::get_initial_state;

    #[test]
    fn it_works() {
        let result =
            get_initial_state("/Users/spg/gameoflife/testfile1.txt").expect("could not get result");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 3);
        assert_eq!(result[1].len(), 3);

        assert_eq!(result[0][0], true);
        assert_eq!(result[0][1], false);
        assert_eq!(result[0][2], true);

        assert_eq!(result[1][0], false);
        assert_eq!(result[1][1], true);
        assert_eq!(result[1][2], false);
    }
}

fn get_file_contents(path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    Ok(contents)
}

fn get_initial_state(path: &str) -> Result<Vec<Vec<bool>>, Box<dyn Error>> {
    let mut max_width = 0;
    let contents = get_file_contents(path)?;

    for line in contents.lines() {
        let cur_len = line.len();
        if cur_len > max_width {
            max_width = cur_len;
        }
    }

    let max_height = contents.lines().count();

    println!("max_height: {max_height}");
    println!("max_width: {max_width}");

    let mut res = Vec::new();

    let lines: Vec<&str> = contents.lines().collect();
    for i in 0..max_height {
        let mut row = Vec::new();
        let line = lines[i];
        let line_length = line.len();

        for c in line.chars() {
            if c == ' ' {
                row.push(false)
            } else {
                row.push(true)
            }
        }

        if max_width > line_length {
            let diff = max_width - line_length;
            for _j in 0..diff {
                row.push(false);
            }
        }

        res.push(row);
    }

    Ok(res)
}
