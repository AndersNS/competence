use crate::competency::path_list::*;
use crate::models::*;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct DisciplineListProps {
    pub disciplines: Vec<Discipline>,
    pub on_rating_changed: Callback<(RatingUpdate, usize, usize, usize, usize)>, // (new rating, competencyId, areaId)
}

#[function_component(DisciplineList)]
pub fn discipline_list(
    DisciplineListProps {
        disciplines,
        on_rating_changed,
    }: &DisciplineListProps,
) -> Html {
    let on_rating_changed = on_rating_changed.clone();
    disciplines
        .iter()
        .map(|disc| {
            let disc = disc.clone();
            let on_rating_changed = on_rating_changed.clone();
            let on_disc_rating_changed = {
                Callback::from(move |pair: (RatingUpdate, usize, usize, usize)| {
                    on_rating_changed.emit((pair.0, pair.1, pair.2, pair.3, disc.id))
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
