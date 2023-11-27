use actix_files as fs;
use actix_web::{web, App, HttpServer};
use fs::Files;
use handlebars::Handlebars;
use pages::{home::home, about::HandleAbout, music::HandleMusic, books::HandleBooks, 
    quotes::HandleQuotes, photos::HandlePhotos, contact::HandleContact};

mod pages {
    pub mod home;
    pub mod about;
    pub mod music;
    pub mod books;
    pub mod quotes;
    pub mod photos;
    pub mod contact;
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
            .route("/contact", web::get().to(HandleContact::show_form))
            .route("/contact", web::post().to(HandleContact::submit_form))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
