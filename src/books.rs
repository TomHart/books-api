use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use chrono::{DateTime, Utc, serde::ts_seconds_option};

#[derive(Serialize, Deserialize)]
struct BooksList {
    books: Books,
}

#[derive(Serialize, Deserialize)]
pub struct Books(pub Vec<Book>);

#[derive(Serialize, Deserialize)]
pub struct Book {
    name: String,
    #[serde(with = "ts_seconds_option")]
    started: Option<DateTime<Utc>>,
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, started at {}", self.name, self.started.expect("Hmm").format("%Y-%m-%d %H:%M:%S"))
    }
}

impl std::fmt::Display for Books {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}

pub fn get() -> Result<Books> {
    let raw_json = fs::read_to_string("data.json").expect("Failed reading data.json").to_owned();
    let books: BooksList = serde_json::from_str(&raw_json)?;

    // let j = serde_json::to_string(&p.books)?;

    Ok(books.books)
}