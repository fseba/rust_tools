use std::io::{BufRead, Result};

// pub fn count_lines(input: impl BufRead) -> Result<usize> {
//     let mut count = 0;
//     for line in input.lines() {
//         line?;
//         count += 1;
//     }
//     Ok(count)
// }
//
pub fn count_lines(input: impl BufRead) -> Result<usize> {
    input
        .lines()
        .try_fold(0, |count, line| line.map(|_| count + 1))
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
    fn count_lines_fn_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count_lines(reader);
        assert!(result.is_err(), "no error returned");
    }

    #[test]
    fn count_lines_counts_lines_in_input() {
        let input = Cursor::new("line 1\nline2\n");
        let lines = count_lines(input).unwrap();
        assert_eq!(lines, 2, "wrong line count");
    }
}
