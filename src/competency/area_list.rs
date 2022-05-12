use crate::competency::competency_list::*;
use crate::models::*;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct AreaListProps {
    pub sub_areas: Vec<Area>,
    pub on_rating_changed: Callback<(u32, usize, usize)>, // (new rating, competencyId, areaId)
}

#[function_component(AreaList)]
pub fn area_list(
    AreaListProps {
        sub_areas,
        on_rating_changed,
    }: &AreaListProps,
) -> Html {
    let on_rating_changed = on_rating_changed.clone();
    sub_areas
        .iter()
        .map(|area| {
            let on_rating_changed = on_rating_changed.clone();
            let area = area.clone();
            let on_area_rating_changed = {
                Callback::from(move |pair: (u32, usize)| {
                    on_rating_changed.emit((pair.0, pair.1, area.id))
                })
            };
            html! {
            <div class="area">
                <h3>{format!("{}", area.name)}</h3>
                <CompetencyList competencies={area.competencies.clone()} on_rating_changed={on_area_rating_changed.clone()} />
             </div>
            }
        })
        .collect()
}
