use crate::common::*;
use crate::models::*;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct CompetencyListProps {
    pub competencies: Vec<Competency>,
    pub on_rating_changed: Callback<CompetencyRating>, // (new rating, competencyId)
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
            let on_rating_changed1 = on_rating_changed.clone();
            let comp = comp.clone();

            let on_interest_changed =
                { 
                Callback::from(move |num: i32| {
                    if num == comp.interest {
                        on_rating_changed.emit(CompetencyRating::new(Rating::Interest(0), comp.id))
                    } else {
                        on_rating_changed.emit(CompetencyRating::new(Rating::Interest(num), comp.id))
                    }
                })
            };
            let on_competency_changed =
                { 
                Callback::from(move |num: i32| {
                    if num == comp.competency {
                        on_rating_changed1.emit(CompetencyRating::new(Rating::Competency(0), comp.id))
                    } else {
                        on_rating_changed1.emit(CompetencyRating::new(Rating::Competency(num), comp.id))
                    }
                })
            };
            html! {

                <div class="competency">
                    <h4>{format!("{}", comp.name)}</h4>
                    <rating::Rating selected={comp.interest} on_click={on_interest_changed.clone()} name={"Interest"}/>
                    <rating::Rating selected={comp.competency} on_click={on_competency_changed.clone()} name={"Competency"}/>
                </div>
            }
        })
        .collect()
}
