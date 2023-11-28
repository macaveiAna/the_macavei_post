use actix_web::{HttpRequest, web};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};

// Contact form Data
#[derive(Debug, serde::Deserialize)]
pub struct ContactForm{
    pub name: String,
    pub email: String,
    pub message: String,

}

// Function for contact form submissions
pub async fn form_submission(form_data: web::Form<ContactFormData>,) -> &'static str {
    
}