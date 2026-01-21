use std::{
    fmt::Display,
    fs::{self, File},
    io::{BufReader, BufWriter, Result},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Memos {
    path: PathBuf,
    pub inner: Vec<Memo>,
}

impl Memos {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut memos = Self {
            path: path.as_ref().to_path_buf(),
            inner: Vec::new(),
        };
        if fs::exists(&path)? {
            let file = BufReader::new(File::open(&path)?);
            // memos.inner = file.lines().collect::<Result<Vec<_>>>()?;
            memos.inner = serde_json::from_reader(file)?;
        }
        Ok(memos)
    }

    pub fn sync(&self) -> Result<()> {
        let file = File::create(&self.path)?;
        serde_json::to_writer(BufWriter::new(file), &self.inner)?;
        Ok(())
    }

    pub fn find_all(&mut self, arg: &str) -> Vec<&mut Memo> {
        self.inner
            .iter_mut()
            .filter(|m| m.status == Status::Pending && m.text.contains(arg))
            .collect()
    }

    pub fn purge_done(&mut self) {
        self.inner.retain(|m| m.status != Status::Done);
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Memo {
    pub text: String,
    pub status: Status,
}

impl Display for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.text)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Pending,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pending => "-",
                Self::Done => "x",
            }
        )
    }
}

#[cfg(test)]
mod test {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn round_trip_via_sync_and_open_preserves_data() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memo.json");
        let memos = Memos {
            path: path.clone(),
            inner: vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Pending,
                },
            ],
        };
        memos.sync().unwrap();
        let memos_2 = Memos::open(&path).unwrap();
        assert_eq!(memos.inner, memos_2.inner, "wrong text");
    }

    #[test]
    fn find_all_return_all_memo_with_matching_text() {
        let mut memos = Memos {
            path: PathBuf::new(),
            inner: vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "_foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "foo".to_string(),
                    status: Status::Done,
                },
            ],
        };
        let found: Vec<&mut Memo> = memos.find_all("foo");
        assert_eq!(found.len(), 2, "wrong number of matches");
        assert_eq!(found[0].text, "foo");
        assert_eq!(found[1].text, "_foo");
    }

    #[test]
    fn purge_done_removes_all_done_tasks() {
        let mut memos = Memos {
            path: PathBuf::new(),
            inner: vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "_foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Done,
                },
                Memo {
                    text: "foo".to_string(),
                    status: Status::Done,
                },
            ],
        };
        memos.purge_done();
        assert_eq!(memos.inner.len(), 2, "wrong number of matches");
        assert_eq!(memos.inner[0].text, "foo");
        assert_eq!(memos.inner[1].text, "_foo");
    }
}
