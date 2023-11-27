use actix_web::{web, HttpResponse, HttpRequest};
use handlebars::Handlebars;

pub struct HandleContact;

impl HandleContact {
    pub async fn show_form(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        let form_content = "Contact Me";
        let body = hb.render("contact_form", &form_content).unwrap();
        HttpResponse::Ok().body(body)
    }

    // Form submission function
    pub async fn submit_form(req: HttpRequest) -> HttpResponse {
        let data_form= Self::extract_form_data(&req);

        //redirect user to a thank-you page
        HttpResponse::SeeOther()
            .append_header((actix_web::http::header::LOCATION, "/thank-you"))
            .finish()
    }

    fn extract_form_data(req: &HttpRequest) -> FormData{

        FormData {
            
        }
    }
}