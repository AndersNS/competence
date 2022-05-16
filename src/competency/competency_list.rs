use crate::common::*;
use crate::models::*;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct CompetencyListProps {
    pub competencies: Vec<Competency>,
    pub on_rating_changed: Callback<(i32, usize)>, // (new rating, competencyId)
}

#[function_component(CompetencyList)]
pub fn competency_list(
    CompetencyListProps {
        competencies,
        on_rating_changed,
    }: &CompetencyListProps,
) -> Html {
    let on_rating_changed = on_rating_changed.clone();
    competencies
        .iter()
        .map(|comp| {
            let on_rating_changed = on_rating_changed.clone();
            let comp = comp.clone();
            let on_comp_rating_changed =
                { Callback::from(move |id: i32| on_rating_changed.emit((id, comp.id))) };
            html! {

                <div class="competency">
                    <h4>{format!("{}", comp.name)}</h4>
                    <rating::Rating selected={comp.interest} on_click={on_comp_rating_changed.clone()} name={"Interest"}/>
                </div>
            }
        })
        .collect()
}
