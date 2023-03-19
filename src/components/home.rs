use yew::{function_component, html, Html};
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <nav class="menu">
            {"Hei! Er du "}
            <Link<Route> to={Route::Discipline { id: 1 }}>{ "Utvikler" }</Link<Route>>
            {" eller kanskje "}
            <Link<Route> to={Route::Discipline { id: 2 }}>{ "Designer" }</Link<Route>>
            {" ?"}
        </nav>
    }
}