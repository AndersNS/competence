use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/discipline/:id")]
    Discipline { id: usize },

    #[not_found]
    #[at("/404")]
    NotFound,
}
