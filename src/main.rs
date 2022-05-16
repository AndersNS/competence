use yew::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod bindings;
pub mod common;
mod competency;
pub mod graph;
pub mod models;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <competency::Competencies />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
