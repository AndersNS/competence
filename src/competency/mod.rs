use crate::common::*;
use crate::models::*;
use gloo_console::log;
use reqwasm::http::Request;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct CompetencyListProps {
    competencies: Vec<Competency>,
}

#[function_component(CompetencyList)]
fn competency_list(CompetencyListProps { competencies }: &CompetencyListProps) -> Html {
    let on_rating_changed = { Callback::from(move |id: u32| log!("Hey", id)) };
    competencies
        .iter()
        .map(|comp| {
            html! {
                <div class="competency">
                    <h4>{format!("{}", comp.name)}</h4>
                    <rating::Rating selected={3} on_click={on_rating_changed.clone()} />
                </div>
            }
        })
        .collect()
}

#[derive(Clone, Properties, PartialEq)]
struct SubAreaListProps {
    sub_areas: Vec<Area>,
}

#[function_component(SubAreaList)]
fn sub_area_list(SubAreaListProps { sub_areas }: &SubAreaListProps) -> Html {
    sub_areas
        .iter()
        .map(|area| {
            html! {
            <div class="area">
                <h3>{format!("{}", area.name)}</h3>
                <CompetencyList competencies={area.competencies.clone()} />
             </div>
            }
        })
        .collect()
}

#[derive(Clone, Properties, PartialEq)]
struct PathListProps {
    paths: Vec<Path>,
}

#[function_component(PathList)]
fn path_list(PathListProps { paths }: &PathListProps) -> Html {
    paths
        .iter()
        .map(|path| {
            html! {
            <div class="path">
                <h2>{format!("{}", path.name)}</h2>
                <SubAreaList sub_areas={path.areas.clone()} />
            </div>
            }
        })
        .collect()
}

#[derive(Clone, Properties, PartialEq)]
struct DisciplineListProps {
    disciplines: Vec<Discipline>,
}

#[function_component(DisciplineList)]
fn discipline_list(DisciplineListProps { disciplines }: &DisciplineListProps) -> Html {
    disciplines
        .iter()
        .map(|prof| {
            let prof = prof.clone();
            html! {
            <>
                <h1>{prof.name}</h1>
                <PathList paths={prof.paths.clone()} />
            </>
            }
        })
        .collect()
}

#[function_component(Competencies)]
pub fn competencies() -> Html {
    let disciplines = use_state(|| vec![]);
    {
        let discs = disciplines.clone();
        use_effect_with_deps(
            move |_| {
                let discs = discs.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_discs: Vec<Discipline> = Request::get("/example.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    discs.set(fetched_discs);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="discipline">
            <DisciplineList disciplines={(*disciplines).clone()} />
        </div>
    }
}
