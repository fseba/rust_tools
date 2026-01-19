use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Result, Write},
    path::Path,
};

fn open(path: impl AsRef<Path>) -> Result<Vec<String>> {
    if fs::exists(&path)? {
        let file = BufReader::new(File::open(&path)?);
        file.lines().collect()
    } else {
        Ok(Vec::new())
    }
}

fn sync(memo: &Vec<String>, path: impl AsRef<Path>) -> Result<()> {
    let mut file = File::options().create(true).append(true).open(path)?;
    writeln!(file, "{}", memo.join("\n"))
}

#[cfg(test)]
mod test {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn open_returns_data_from_given_file() {
        let memos = open("./tests/data/test_file_1").unwrap();
        assert_eq!(memos, vec!["foo", "bar"], "wrong data");
    }

    #[test]
    fn open_returns_empty_vec_for_missing_file() {
        let memos = open("bogus.txt").unwrap();
        assert!(memos.is_empty(), "vec not empty");
    }

    #[test]
    fn sync_creates_file_if_necessary() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memo.txt");
        let memos = vec!["foo".to_string(), "bar".to_string()];
        sync(&memos, &path).unwrap();
        let text = fs::read_to_string(path).unwrap();
        assert_eq!(text, "foo\nbar\n", "wrong text");
    }

    #[test]
    fn sync_appends_to_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memo.txt");
        fs::write(&path, "foo\n").unwrap();
        let memo = vec!["bar".to_string()];
        sync(&memo, &path).unwrap();
        let text = fs::read_to_string(path).unwrap();
        assert_eq!(text, "foo\nbar\n", "wrong text");
    }
}
