/*
 * /src/pages/home.rs
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
                "title":"I much prefer the sharpest criticism of a single intelligent man to the thoughtless approval of the masses. - Johannes Kepler",
            },
        ]
    });
    // My logic for handling posts
    let body = hb.render("posts", &posts).unwrap();
    HttpResponse::Ok().body(body)
}
/* 
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, http};

    #[tokio::test]
    async fn test_home() {
        //create app
        let mut app = test::init_service(App::new().route("/", web::get().to(home))).await;

        //send a get request
        let req = test::TestRequest::get().uri("/").to_request();
        let respond = test::call_service(&mut app, req).await;

        assert_eq!(respond.status(), http::StatusCode::OK);
    }
}*/
