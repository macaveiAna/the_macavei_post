/* 
*  Ana Macavei
*  CS410P Rust
*  11/28/2023
*  This is the file where the main function is run
*/

use actix_web::{web, App, HttpServer};
use actix_files::Files;
use handlebars::Handlebars;
use pages::{home::home, about::HandleAbout, music::HandleMusic, books::HandleBooks, 
    quotes::HandleQuotes, photos::HandlePhotos};
/*use sqlx::{PgPool, Postgres, Result};
#[macro_use]
extern crate sqlx;
#[derive(sqlx::FromRow)]
struct User {
    username: String,
    password: String,
}*/
mod pages {
    pub mod home;
    pub mod about;
    pub mod music;
    pub mod books;
    pub mod quotes;
    pub mod photos;
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

/* 
// function to initialize database
pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect("postgresql://username:password@localhost/database")
        .await?();
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    Ok(pool)
}  */