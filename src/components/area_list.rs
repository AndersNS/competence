use crate::components::competency_list::*;
use crate::graph::*;
use crate::models::*;
use gloo_console::log;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct AreaListProps {
    pub path_id: usize,
    pub sub_areas: Vec<Area>,
    pub on_rating_changed: Callback<CompetencyRating>, // (new rating, competencyId, areaId)
}

#[function_component(AreaList)]
pub fn area_list(
    AreaListProps {
        path_id,
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
                Callback::from(move |mut rating: CompetencyRating| {
                    rating.area_id = area.id;
                    on_rating_changed.emit(rating)
                })
            };

            let interest: Vec<i32> = area.competencies.iter().map(|c| c.interest).collect();
            let competency: Vec<i32> = area.competencies.iter().map(|c| c.competency).collect();
            let labels: Vec<String> = area
                .competencies
                .iter()
                .map(|c| c.name.to_string())
                .collect();

            let on_area_selected_callback = { Callback::from(move |_| log!("ey")) };
            html! {
            <div class="area">
                <div>
                    <h3 onclick={on_area_selected_callback.clone()}>{format!("{}", area.name)}</h3>
                    <CompetencyList
                        competencies={area.competencies.clone()}
                        on_rating_changed={on_area_rating_changed.clone()}
                        />
                </div>
                <div>
                    <Graph
                        id={format!("graph-{}-{}", area.name.to_lowercase(), path_id)}
                        interest={interest.clone()}
                        competency={competency.clone()}
                        labels={labels.clone()} />
                </div>
             </div>
            }
        })
        .collect()
}
