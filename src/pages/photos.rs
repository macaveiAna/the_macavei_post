use actix_web::{web, HttpResponse};
use serde_json::json;
use handlebars::Handlebars;

pub struct HandlePhotos;

impl HandlePhotos {
    pub async fn photos(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        //my logic for the photos page
        let photo_content = json!({
            "content": "My Photos goes here...",
        });
        let body =hb.render("photos", &photo_content).unwrap();
        HttpResponse::Ok().body(body)
    }
}