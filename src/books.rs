pub mod structs;

use std::fs;
use uuid::Uuid;
use crate::books::structs::{Book, Books, BooksList};

pub fn get() -> Books {
    let raw_json = fs::read_to_string("data.json").expect("Failed reading data.json").to_owned();
    let books: BooksList = serde_json::from_str(&raw_json).unwrap();

    books.books
}

pub fn add_book(mut book: Book) -> Books {
    println!("Name: {}", book.name);

    if book.id.is_none() {
        book.id = Some(Uuid::new_v4().to_string());
    }

    let raw_json = fs::read_to_string("data.json").expect("Failed reading data.json").to_owned();
    let mut books: BooksList = serde_json::from_str(&raw_json).unwrap();

    let index_of_first_even_number = books.books.0.iter().position(|x| x.id == book.id);

    if index_of_first_even_number.is_none() {
        books.books.0.push(book);
    } else {
        books.books.0[index_of_first_even_number.unwrap()] = book;
    }

    let string: String = serde_json::to_string_pretty(&books).expect("Err");
    fs::write("data.json", string).expect("Unable to write to data.json");

    books.books
}

pub fn get_by_id(book_id: String) -> Result<Book, String>
{
    let raw_json = fs::read_to_string("data.json").expect("Failed reading data_.json").to_owned();
    let books: BooksList = serde_json::from_str(&raw_json).unwrap();

    let tmp: Vec<Book> = books.books.0;

    for book in tmp {
        let id = book.id.as_ref();
        if !id.is_none() && id.unwrap() == &book_id {
            return Ok(book);
        }
    }

    Err("Not found".to_string())
}
