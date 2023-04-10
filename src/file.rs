use std::{
    env,
    error::Error,
    fs,
    path::{self, Path},
};

#[cfg(test)]
mod tests {
    use std::path;

    use crate::file::get_initial_state;

    #[test]
    fn testfile1() {
        let cd = std::env::current_dir().expect("could not get current dir");
        let binding = cd.as_path().join(path::Path::new("testfile1.txt"));

        let testfile = &binding;

        let result = get_initial_state(testfile).expect("could not get result");
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
    fn testfile2() {
        let cd = std::env::current_dir().expect("could not get current dir");
        let binding = cd.as_path().join(path::Path::new("testfile2.txt"));

        let testfile = &binding;

        let result = get_initial_state(testfile).expect("could not get result");
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

fn get_initial_state(path: &Path) -> Result<Vec<Vec<bool>>, Box<dyn Error>> {
    let mut max_width = 0;
    let contents = get_file_contents(path)?;

    for line in contents.lines() {
        let cur_len = line.len();
        if cur_len > max_width {
            max_width = cur_len;
        }
    }

    let max_height = contents.lines().count();

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
