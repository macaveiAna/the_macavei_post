/**
 * src/pages/photos.rs
 * This file handles the photography content
 *
*/
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

pub struct HandlePhotos;

// A function to handle photography content
impl HandlePhotos {
    pub async fn photos(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        //my logic for the photos page
        let photo_content = json!({
            "photo_content": [
                {
                    "image_path":"/static/img/Scenic.jpg"
                },
            ]
        });
        let body = hb.render("photos", &photo_content).unwrap();
        HttpResponse::Ok().body(body)
    }
}
