/**
 * src/pages/books.rs
 * This file handles the books content
 *
*/
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

pub struct HandleBooks;

//A function that encapsulates my book content
impl HandleBooks {
    pub async fn library(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        //my logic for the books page
        let book_content = json!({
            "book_content": [
                {
                    "name":"Pride and Predjudice",
                    "author": "Jane Austen",
                    "image_path": "/static/img/book1.png"
                },
                {
                    "name": "The Knowledge of the Holy",
                    "author": "A.W Tozer",
                    "image_path": "/static/img/book2.png"
                },
            ]
        });
        let body = hb.render("book_content", &book_content).unwrap();
        HttpResponse::Ok().body(body)
    }
}
