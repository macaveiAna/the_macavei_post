use actix_web::{web, HttpResponse};
use serde_json::json;
use handlebars::Handlebars;

pub struct HandleMusic;

impl HandleMusic {
    pub async fn music(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        //my logic for the music page
        let music_content = json!({
            "content": "Music goes here...",
        });
        let body =hb.render("music", &music_content).unwrap();
        HttpResponse::Ok().body(body)
    }
}