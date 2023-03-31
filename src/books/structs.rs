use chrono::{DateTime, Utc, serde::ts_seconds_option};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BooksList {
    pub books: Books,
}

#[derive(Serialize, Deserialize)]
pub struct Books(pub Vec<Book>);

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub id: Option<String>,
    #[serde(with = "ts_seconds_option")]
    pub started: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub finished: Option<DateTime<Utc>>,
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let started = match self.started {
            Some(date) => format!("started at {}", date.format("%Y-%m-%d %H:%M:%S").to_string()),
            None => "not started yet".to_string()
        };

        write!(f, "{}, {}", self.name, started)
    }
}

impl std::fmt::Display for Books {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.iter().fold(Ok(()), |result, album| {
            result.and_then(|_| writeln!(f, "{}", album))
        })
    }
}