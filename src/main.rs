use actix_files as fs;
use actix_web::{web, App, HttpServer};
use fs::Files;
use handlebars::Handlebars;
use pages::{home::home, about::HandleAbout};

mod pages {
    pub mod home;
    pub mod about;
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
            // New rout for about page
            .route("/about", web::get().to(HandleAbout::about))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
