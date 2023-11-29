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
                    "text": "If we find ourselves with a desire that nothing in this world can satisfy, the most probable explanation is that we were made for another world.",
                    "author": "C.S Lewis"
                },
                {
                    "text": "It's not that I'm so smart, it's just that I stay with problems longer.",
                    "author":"Albert Einstein"
                }
            ]
        });
        let body =hb.render("quotes", &quote_content).unwrap();
        HttpResponse::Ok().body(body)
    }
}