extern crate tempfile;
extern crate rev_lines;

use tempfile::tempfile;
use rev_lines::RevLines;

use std::fs::File;
use std::io::{
    Write, Result, Error, ErrorKind
};
use std::io::BufReader;

/// Naive and simple key,value database as illustrated in DDIA Chapter 3 using simple bash script.
pub struct Db {
    file: File
}

impl Db {
    pub fn temp() -> Result<Self> {
        tempfile().map(|tf| Db { file: tf })
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        self.file.write_fmt(format_args!("{},{}\n", key, value))
    }

    pub fn get(&self, key: &str) -> Result<String> {
        self.file.try_clone()
            .and_then(|f| {
                RevLines::new(BufReader::new(f))
                    .and_then(|mut rl| {
                        rl.find_map(|line| {
                            Some(line.split(",").collect())
                                .filter(|split_vec: &Vec<&str>| {
                                    split_vec.len() == 2 && split_vec.get(0)
                                        .map(|k: &&str| *k == key)
                                        .unwrap_or(false)
                                })
                                .and_then(|split_vec: Vec<&str>| {
                                    split_vec.get(1).map(|s: &&str| String::from(*s))
                                })
                        }).ok_or(Error::from(ErrorKind::NotFound))
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_value() {
        let value = Db::temp()
            .and_then(|mut db| {
                db.set("key", "value")
                    .map(|_| db)
            })
            .and_then(|db| {
                db.get("key")
            })
            .expect("expected value");
        assert_eq!("value", value);
    }

    #[test]
    fn test_latest_wins() {
        let value = Db::temp()
            .and_then(|mut db| {
                db.set("key", "value")
                    .map(|_| db)
            })
            .and_then(|mut db| {
                db.set("key", "value_new")
                    .map(|_| db)
            })
            .and_then(|db| {
                db.get("key")
            })
            .expect("expected value");
        assert_eq!("value_new", value);
    }
}

