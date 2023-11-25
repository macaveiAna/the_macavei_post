//use crate::view::home::Home;
use yew::prelude::*;
//use yew_router::prelude::*;
// use log::info;
//use router::App;
//mod pages::home::Home; 
use actix_web::{web,HttpServer,HttpResponse,App};
use actix_files::{Files};
use serde_json::json;
use handlebars::Handlebars;

/* 
// Reference this in main
#[function_component(App)]
fn app() -> Html {

    //info!("Starting app...");
    html! {
        //<>
        <h1>{"Hello World!"}</h1>
       
        //</>
        
    }
}
 */

#[actix_web::main]
 fn main() -> std::io::Result<()>
 {
    
    //yew::Renderer::<App>::new().render();
}
