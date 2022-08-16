use crate::components::path_list::*;
use crate::models::*;
use crate::store::State;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Clone, Properties, PartialEq)]
pub struct DisciplineListProps {}

#[function_component(DisciplineList)]
pub fn discipline_list(DisciplineListProps {}: &DisciplineListProps) -> Html {
    let (store, dispatch) = use_store::<State>();

    let on_disc_rating_changed = {
        Callback::from(move |rating: CompetencyRating| {
            dispatch.reduce_mut(|state| state.update_rating(rating));
        })
    };

    store
        .disciplines
        .iter()
        .map(|disc| {
            let disc = disc.clone();
            let on_rating_changed = on_disc_rating_changed.clone();
            let on_disc_rating_changed = {
                Callback::from(move |mut rating: CompetencyRating| {
                    rating.discipline_id = disc.id;
                    on_rating_changed.emit(rating)
                })
            };
            html! {
            <>
                <h1>{disc.name}</h1>
                <PathList
                    paths={disc.paths.clone()}
                    on_rating_changed={on_disc_rating_changed.clone()}
                />
            </>
            }
        })
        .collect()
}
