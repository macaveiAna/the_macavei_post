/*
 * Serves as the starting page of the website
 */
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

// My home function

pub async fn home(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let posts = json!({
        "posts": [
            {
                "title":"My Favorite Books",
                "image_path":"/img/img1.png"
            },
        ]
    });
    // My logic for handling posts
    let body = hb.render("posts", &posts).unwrap();
    HttpResponse::Ok().body(body)
}
