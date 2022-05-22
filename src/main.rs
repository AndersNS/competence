use routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod bindings;
pub mod common;
mod competency;
pub mod graph;
pub mod models;
pub mod routes;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <competency::Competencies /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
