//use crate::view::home::Home;
use yew::prelude::*;
//use yew_router::prelude::*;
// use log::info;
//use router::App;
//mod pages::home::Home; 


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
 

fn main() {
    // wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
