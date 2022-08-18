use routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod bindings;
mod components;
pub mod graph;
pub mod local_storage;
pub mod models;
pub mod routes;
pub mod store;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <components::Competencies /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
