use actix_web::{web, HttpResponse};
use serde_json::json;
use handlebars::Handlebars;

pub struct HandleAbout;

impl HandleAbout {
    pub async fn about(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        //my logic for the about page
        let about_content = json!({
            "content": "About me...",
        });
        let body =hb.render("about", &about_content).unwrap();
        HttpResponse::Ok().body(body)
    }
}