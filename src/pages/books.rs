use actix_web::{web, HttpResponse};
use serde_json::json;
use handlebars::Handlebars;

pub struct HandleBooks;

impl HandleBooks {
    pub async fn library(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        //my logic for the books page
        let book_content = json!({
            "content": "My Library goes here...",
        });
        let body =hb.render("books", &book_content).unwrap();
        HttpResponse::Ok().body(body)
    }
}