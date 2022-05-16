use crate::competency::competency_list::*;
use crate::models::*;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct AreaListProps {
    pub sub_areas: Vec<Area>,
    pub on_rating_changed: Callback<(i32, usize, usize)>, // (new rating, competencyId, areaId)
    pub on_area_selected: Callback<usize>,
}

#[function_component(AreaList)]
pub fn area_list(
    AreaListProps {
        sub_areas,
        on_rating_changed,
        on_area_selected,
    }: &AreaListProps,
) -> Html {
    let on_rating_changed = on_rating_changed.clone();
    let on_area_selected = on_area_selected.clone();
    sub_areas
        .iter()
        .map(|area| {
            let on_rating_changed = on_rating_changed.clone();
            let area = area.clone();
            let on_area_rating_changed = {
                Callback::from(move |pair: (i32, usize)| {
                    on_rating_changed.emit((pair.0, pair.1, area.id))
                })
            };

            let on_area_selected = on_area_selected.clone();
            let on_area_selected_callback =
                { Callback::from(move |_| on_area_selected.emit(area.id)) };
            html! {
            <div class="area">
                <h3 onclick={on_area_selected_callback.clone()}>{format!("{}", area.name)}</h3>
                <CompetencyList
                    competencies={area.competencies.clone()}
                    on_rating_changed={on_area_rating_changed.clone()}
                    />
             </div>
            }
        })
        .collect()
}
