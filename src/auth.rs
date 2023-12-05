/*use actix_web::{web, HttpResponse, Result};
use actix_identity::Identity;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginCreds {
    pub username: String,
    pub password: String,
}

// login handler function
pub async fn login(identity: Identity, form: web::Form<LoginCreds>) -> HttpResponse {
    //first check username and password
    if form.username == "macavei" && form.password == "123456" {
        //success!!!
        identity.remember("user".to_string());
        HttpResponse::SeeOther().header("location", "/").finish()
    } else {
        //failure...
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}

pub async fn logout(identity:Identity) -> Result<HttpResponse> {
    identity.forget();
    Ok(HttpResponse::SeeOther().append_header(("location", "/")).finish())
}

pub async fn login_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("static/login.html"))
}*/