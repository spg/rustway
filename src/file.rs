use std::{cmp::min, error::Error, fs, path::Path};

#[cfg(test)]
mod tests {
    use std::path;

    use crate::file::get_initial_state;

    #[test]
    fn testfile1_1() {
        let cd = std::env::current_dir().expect("could not get current dir");
        let binding = cd.as_path().join(path::Path::new("testfile1.txt"));

        let testfile = &binding;

        let result = get_initial_state(testfile, 2, 3).expect("could not get result");
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

    #[test]
    fn testfile1_2() {
        let cd = std::env::current_dir().expect("could not get current dir");
        let binding = cd.as_path().join(path::Path::new("testfile1.txt"));

        let testfile = &binding;

        let result = get_initial_state(testfile, 5, 5).expect("could not get result");
        assert_eq!(result.len(), 5);
        for row in &result {
            assert_eq!(row.len(), 5)
        }

        assert_eq!(result[0][0], true);
        assert_eq!(result[0][1], false);
        assert_eq!(result[0][2], true);
        assert_eq!(result[0][3], false);
        assert_eq!(result[0][4], false);

        assert_eq!(result[1][0], false);
        assert_eq!(result[1][1], true);
        assert_eq!(result[1][2], false);
        assert_eq!(result[1][3], false);
        assert_eq!(result[1][4], false);

        // all the rest is false
        for i in 2..5 {
            for j in 0..5 {
                assert_eq!(result[i][j], false)
            }
        }
    }

    #[test]
    fn testfile1_3() {
        let cd = std::env::current_dir().expect("could not get current dir");
        let binding = cd.as_path().join(path::Path::new("testfile1.txt"));

        let testfile = &binding;

        let result = get_initial_state(testfile, 2, 2).expect("could not get result");
        assert_eq!(result.len(), 2);
        for row in &result {
            assert_eq!(row.len(), 2)
        }

        assert_eq!(result[0][0], true);
        assert_eq!(result[0][1], false);

        assert_eq!(result[1][0], false);
        assert_eq!(result[1][1], true);
    }

    #[test]
    fn testfile2() {
        let cd = std::env::current_dir().expect("could not get current dir");
        let binding = cd.as_path().join(path::Path::new("testfile2.txt"));

        let testfile = &binding;

        let result = get_initial_state(testfile, 4, 7).expect("could not get result");
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].len(), 7);
        assert_eq!(result[1].len(), 7);
        assert_eq!(result[2].len(), 7);
        assert_eq!(result[3].len(), 7);

        // line 1
        assert_eq!(result[0][0], true);
        assert_eq!(result[0][1], false);
        assert_eq!(result[0][2], true);
        assert_eq!(result[0][3], false);
        assert_eq!(result[0][4], true);
        assert_eq!(result[0][5], false);
        assert_eq!(result[0][6], false);

        // line 2
        assert_eq!(result[1][0], true);
        assert_eq!(result[1][1], false);
        assert_eq!(result[1][2], false);
        assert_eq!(result[1][3], false);
        assert_eq!(result[1][4], false);
        assert_eq!(result[1][5], false);
        assert_eq!(result[1][6], false);

        // line 3
        assert_eq!(result[2][0], false);
        assert_eq!(result[2][1], false);
        assert_eq!(result[2][2], false);
        assert_eq!(result[2][3], false);
        assert_eq!(result[2][4], false);
        assert_eq!(result[2][5], false);
        assert_eq!(result[2][6], true);

        // line 4
        assert_eq!(result[3][0], false);
        assert_eq!(result[3][1], false);
        assert_eq!(result[3][2], true);
        assert_eq!(result[3][3], false);
        assert_eq!(result[3][4], false);
        assert_eq!(result[3][5], false);
        assert_eq!(result[3][6], false);
    }
}

fn get_file_contents(path: &Path) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    Ok(contents)
}

pub fn get_initial_state(
    path: &Path,
    n_rows: usize,
    n_cols: usize,
) -> Result<Vec<Vec<bool>>, Box<dyn Error>> {
    let contents = get_file_contents(path)?;
    let mut res = Vec::new();

    let lines: Vec<&str> = contents.lines().collect();
    for i in 0..n_rows {
        let mut row = Vec::new();
        if i < lines.len() {
            let line = lines[i];
            let line_length = line.len();
            let slice_len = min(n_cols, line_length);
            let slice = &line[..slice_len];

            for c in slice.chars() {
                if c == ' ' {
                    row.push(false)
                } else {
                    row.push(true)
                }
            }

            if n_cols > line_length {
                let diff = n_cols - line_length;
                for _j in 0..diff {
                    row.push(false);
                }
            }
        } else {
            for _i in 0..n_cols {
                row.push(false);
            }
        }

        res.push(row);
    }

    Ok(res)
}
