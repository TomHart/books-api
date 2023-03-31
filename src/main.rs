mod books;

use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder, http::header};
use std::fs;
use json;
use crate::books::structs::{Book, Books};

#[get("/books")]
async fn books_route() -> impl Responder {
    let books: Books = books::get();
    let string: String = serde_json::to_string(&books).expect("Err");

    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body(string)
}

#[get("/books/{book_id}")]
async fn index(path: web::Path<String>) -> impl Responder {
    let book_id = path.into_inner();

    let book_result = books::get_by_id(book_id);

    let book: Book = match book_result {
        Ok(book) => book,
        Err(_err) => return HttpResponse::NotFound().finish()
    };

    let string: String = serde_json::to_string(&book).expect("Err");

    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body(string)
}

#[put("/books")]
async fn add_book(form: web::Json<Book>) -> impl Responder {
    let books: Books = books::add_book(form.into_inner());
    let string: String = serde_json::to_string_pretty(&books).expect("Err");
    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body(string)
}


fn init_json() -> bool {
    let raw_json = fs::read_to_string("data.json").unwrap_or_else(|_error| {
        return create_json();
    });

    json::parse(&raw_json).unwrap_or_else(|_error| {
        return json::parse(&create_json()).unwrap();
    });

    return true;
}

fn create_json() -> String {
    fs::OpenOptions::new().write(true)
        .create_new(true)
        .open("data.json").expect("Couldn't create data.json");

    fs::write("data.json", "{\"books\": []}").expect("Unable to write to data.json");

    return fs::read_to_string("data.json").unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !init_json() {
        println!("Can't create data.json");
    }

    println!("Starting");
    HttpServer::new(|| {
        App::new()
            .service(books_route)
            .service(add_book)
            .service(index)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}