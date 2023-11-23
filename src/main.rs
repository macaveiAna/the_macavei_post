use crate::view::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;
use log::info;

#[function_component(App)]
fn app() -> Html {

    info!("Starting app...");
    html! {
        <>
            //<Home />

            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
        
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
