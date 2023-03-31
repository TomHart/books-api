mod books;

use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder, http::header};
use std::fs::OpenOptions;
use std::fs;
use json;
use crate::books::structs::{Book, Books};

#[get("/books")]
async fn books_route() -> impl Responder {
    let books: Books = books::get().expect("Getting error");
    let string: String = serde_json::to_string(&books).expect("Err");

    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body(string)
}

#[put("/books")]
async fn add_book(form: web::Json<Book>) -> impl Responder {
    let books: Books = books::add_book(form.into_inner()).expect("Error adding book");
    let string: String = serde_json::to_string_pretty(&books).expect("Err");
    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body(string)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
    OpenOptions::new().write(true)
        .create_new(true)
        .open("data.json").expect("Couldn't create data.json");

    fs::write("data.json", "{\"books\": []}").expect("Unable to write to data.json");

    return fs::read_to_string("data.json").unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !init_json() {
        println!("Can't create data_.json");
    }

    // books::get().expect("Something failed");
    // println!("Books: {}", books::get().expect("Err"));

    println!("Started");
    HttpServer::new(|| {
        App::new()
            .service(books_route)
            .service(add_book)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}