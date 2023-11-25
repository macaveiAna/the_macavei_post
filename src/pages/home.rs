/*
 * Serves as the starting page of the website
 */
use actix_web::{web,HttpServer,HttpResponse,App};
use actix_files::{Files};
use serde_json::json;
use handlebars::Handlebars;


fn home(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    
}