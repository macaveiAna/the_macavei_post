use actix_web::{web, HttpResponse};
use handlebars::Handlebars;

pub struct HandleContact;

impl HandleContact {
    pub async fn show_form(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        let form_content = "Contact Me";
        let body = hb.render("contact_form", &form_content).unwrap();
        HttpResponse::Ok().body(body)
    }

    // Form submission function
    pub async fn submit_form() -> HttpResponse {

        //redirect user to a thank-you page
        HttpResponse::SeeOther()
            .header(actix_web::http::header::LOCATION, "/thank-you")
            .finish()
    }
}