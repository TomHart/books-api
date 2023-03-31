pub mod structs;

use serde_json::Result;
use std::fs;
use uuid::Uuid;
use crate::books::structs::{Book, Books, BooksList};

pub fn get() -> Result<Books> {
    let raw_json = fs::read_to_string("data.json").expect("Failed reading data_.json").to_owned();
    let books: BooksList = serde_json::from_str(&raw_json)?;

    Ok(books.books)
}

pub fn add_book(mut book: Book) -> Result<Books> {
    println!("Name: {}", book.name);

    if book.id.is_none() {
        book.id = Some(Uuid::new_v4().to_string());
    }

    println!("ID:   {:?}", book.id);

    let raw_json = fs::read_to_string("data.json").expect("Failed reading data_.json").to_owned();
    let mut books: BooksList = serde_json::from_str(&raw_json)?;

    let index_of_first_even_number = books.books.0.iter().position(|x| x.id == book.id);
    println!("{:?}", index_of_first_even_number);

    if index_of_first_even_number.is_none() {
        books.books.0.push(book);
    } else {
        books.books.0[index_of_first_even_number.unwrap()] = book;
    }

    let string: String = serde_json::to_string_pretty(&books).expect("Err");
    fs::write("data.json", string).expect("Unable to write to data.json");

    Ok(books.books)
}