use crate::pages::{home::Home, about::About};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/Not Found")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match routes {
        Route::Home => html! { <Home />},
        Rout::About => html! {<About />},
        Route::NotFound => html! { <h1>{ "Not Found" }</h1> },
    }
}