use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

#[derive(Default)]
pub struct Count {
    pub lines: usize,
    pub words: usize,
}

pub fn count_lines(mut input: impl BufRead) -> Result<usize> {
    let mut count = 0;
    let mut line = String::new();
    while input.read_line(&mut line)? > 0 {
        count += 1;
        line.clear();
    }
    Ok(count)
}

// pub fn count_lines(input: impl BufRead) -> Result<usize> {
//     input
//         .lines()
//         .try_fold(0, |count, line| line.map(|_| count + 1))
// }

pub fn count_in_path(path: &String) -> Result<Count> {
    let file = File::open(path).with_context(|| path.clone())?;
    let file = BufReader::new(file);
    count(file).with_context(|| path.clone())
}

fn count(mut input: impl BufRead) -> Result<Count> {
    let mut count = Count::default();
    let mut line = String::new();
    while input.read_line(&mut line)? > 0 {
        count.lines += 1;
        count.words += line.split_whitespace().count();
        line.clear();
    }
    Ok(count)
}

#[cfg(test)]
mod test {
    use std::io::{BufReader, Cursor, Error, Read, Result};

    use super::*;

    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
            Err(Error::other("oh no"))
        }
    }

    #[test]
    fn count_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count(reader);
        assert!(result.is_err(), "no error returned");
    }

    #[test]
    fn count_lines_counts_lines_in_input() {
        let input = Cursor::new("line 1\nline2\n");
        let lines = count_lines(input).unwrap();
        assert_eq!(lines, 2, "wrong line count");
    }

    #[test]
    fn count_in_path_counts_lines_and_words_in_given_file() {
        let count = count_in_path(&"./src/test_file".to_string()).unwrap();
        assert_eq!(count.lines, 2, "wrong line count");
        assert_eq!(count.words, 4, "wrong word count");
    }

    #[test]
    fn count_counts_words_and_lines_in_input() {
        let input = Cursor::new("one two\nthree");
        let count = count(input).unwrap();
        assert_eq!(count.lines, 2, "wrong line count");
        assert_eq!(count.words, 3, "wrong word count");
    }
}
