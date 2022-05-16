use crate::competency::path_list::*;
use crate::models::*;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct DisciplineListProps {
    pub disciplines: Vec<Discipline>,
    pub on_rating_changed: Callback<(i32, usize, usize, usize, usize)>, // (new rating, competencyId, areaId)
    pub on_area_selected: Callback<(usize, usize, usize)>, // (area_id, path_id, discipline_id)
}

#[function_component(DisciplineList)]
pub fn discipline_list(
    DisciplineListProps {
        disciplines,
        on_rating_changed,
        on_area_selected,
    }: &DisciplineListProps,
) -> Html {
    let on_rating_changed = on_rating_changed.clone();
    let on_area_selected = on_area_selected.clone();
    disciplines
        .iter()
        .map(|disc| {
            let disc = disc.clone();
            let on_rating_changed = on_rating_changed.clone();
            let on_area_selected = on_area_selected.clone();
            let on_disc_rating_changed = {
                Callback::from(move |pair: (i32, usize, usize, usize)| {
                    on_rating_changed.emit((pair.0, pair.1, pair.2, pair.3, disc.id))
                })
            };
            let on_area_selected_callback = {
                Callback::from(move |pair: (usize, usize)| {
                    on_area_selected.emit((pair.0, pair.1, disc.id))
                })
            };
            html! {
            <>
                <h1>{disc.name}</h1>
                <PathList
                    paths={disc.paths.clone()}
                    on_rating_changed={on_disc_rating_changed.clone()}
                    on_area_selected={on_area_selected_callback.clone()}/>
            </>
            }
        })
        .collect()
}
