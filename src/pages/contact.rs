use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde::Deserialize;
use crate::email::EmailService;

#[derive(Debug, Deserialize)]
pub struct ContactForm {
    name: String,
    email: String,
    message: String,
}
pub struct HandleContact;

impl HandleContact {
    pub async fn show_form(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        let form_content = "Contact Me";
        let body = hb.render("contact_form", &form_content).unwrap();
        HttpResponse::Ok().body(body)
    }
    // Form submission function
    pub async fn submit_form(form_data: web::Form<ContactForm>) -> HttpResponse {
        //let data_form= Self::extract_form_data();
        println!("Received form: {:?}", form_data);

        //redirect user to a thank-you page
        HttpResponse::SeeOther()
            .append_header((actix_web::http::header::LOCATION, "/thank-you"))
            .finish()
    }

    /* 
    fn extract_form_data(req: &HttpRequest) -> FormData{

        FormData {
            
        }
    }
    */
}