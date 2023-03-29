use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header};
use std::fs::OpenOptions;
use std::fs;
use std::path::Path;
use json;
use crate::books::Books;

mod books;

#[get("/a")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body(req_body)
}

#[get("/books")]
async fn books_route() -> impl Responder {
    let books: Books = books::get().expect("Getting error");
    let string: String = serde_json::to_string(&books).expect("Err");
    HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .body(string)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn init_json() -> bool {
    if !Path::new("data.json").exists() {
        create_json();
    }

    let raw_json = fs::read_to_string("data.json").expect("Failed reading data.json");
    let parsed = json::parse(&raw_json);

    let output = match parsed {
        Ok(_json) => true,
        Err(_err) => create_json()
    };

    return output;
}

fn create_json() -> bool {
    let _file = OpenOptions::new().write(true)
        .create_new(true)
        .open("data.json");

    fs::write("data.json", "{\"books\": []}").expect("Unable to write file");

    return true;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !init_json() {
        println!("Can't create data.json");
    }

    // books::get().expect("Something failed");
    println!("Books: {}", books::get().expect("Err"));

    HttpServer::new(|| {
        App::new()
            .service(books_route)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}