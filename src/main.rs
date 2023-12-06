/*
*  Ana Macavei
*  CS410P Rust
*  11/28/2023
*  This is the file where the main function is run
*/

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use handlebars::Handlebars;
use pages::{
    about::HandleAbout, books::HandleBooks, home::home, music::HandleMusic, photos::HandlePhotos,
    quotes::HandleQuotes,
};

mod pages {
    pub mod about;
    pub mod books;
    pub mod home;
    pub mod music;
    pub mod photos;
    pub mod quotes;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut hbars = Handlebars::new();
    hbars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let hbars_ref = web::Data::new(hbars);

    HttpServer::new(move || {
        App::new()
            .app_data(hbars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(home))
            .route("/about", web::get().to(HandleAbout::about))
            .route("/music", web::get().to(HandleMusic::music))
            .route("/books", web::get().to(HandleBooks::library))
            .route("/quotes", web::get().to(HandleQuotes::quotes))
            .route("/photos", web::get().to(HandlePhotos::photos))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};

    #[tokio::test]
    async fn test_home() {
        //create app
        let mut app = test::init_service(App::new().route("/", web::get().to(home))).await;

        //send a get request
        let req = test::TestRequest::get().uri("/").to_request();
        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), http::StatusCode::OK);

        //check to make sure the home page holds correct contents
        let body = test::read_body(res).await;
        assert!(body
            .windows("I much prefer the sharpest criticism".len())
            .any(|window| window == "I much prefer the sharpest criticism".as_bytes()));
    }
}

