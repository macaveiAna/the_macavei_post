use actix_web::{web, HttpResponse};
use serde_json::json;
use handlebars::Handlebars;

pub struct HandleQuotes;

impl HandleQuotes {
    pub async fn quotes(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        //my logic for the quotes page
        let quote_content = json!({
            "content": [
                {
                    "text": "first quote",
                    "author": "name"
                },
                {
                    "text": "another quote",
                    "author":"name"
                }
            ]
        });
        let body =hb.render("quotes", &quote_content).unwrap();
        HttpResponse::Ok().body(body)
    }
}