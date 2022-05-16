use crate::competency::discipline_list::*;
use crate::graph::*;
use crate::models::*;
use gloo_console::log;
use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

mod area_list;
mod competency_list;
mod discipline_list;
mod path_list;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
struct CompetencyInterest {
    discipline_id: usize,
    path_id: usize,
    area_id: usize,
    comp_id: usize,
    interest: i32,
}

#[function_component(Competencies)]
pub fn competencies() -> Html {
    let disciplines = use_state(|| vec![]);
    let selected_area: UseStateHandle<Option<Area>> = use_state(|| None);
    {
        let discs = disciplines.clone();
        use_effect_with_deps(
            move |_| {
                let discs = discs.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut fetched_discs: Vec<Discipline> = Request::get("/example.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    // Loop over stored values and update the state
                    let competencies: Result<Vec<CompetencyInterest>, StorageError> =
                        LocalStorage::get("competencies");
                    match competencies {
                        Ok(comps) => {
                            for ele in comps {
                                if let Some(index) =
                                    fetched_discs.iter().position(|c| c.id == ele.discipline_id)
                                {
                                    fetched_discs[index].set_interest(
                                        ele.interest,
                                        ele.path_id,
                                        ele.area_id,
                                        ele.comp_id,
                                    )
                                }
                            }
                        }
                        _ => {}
                    }

                    discs.set(fetched_discs);
                });
                || ()
            },
            (),
        );
    }

    let on_disc_rating_changed = {
        // TODO This is a huge mess
        // Refactor so we dont use a giant tuple
        // Maybe add an impementation to CompKey from tuple, or something else
        let disciplines = disciplines.clone();
        let selected_area = selected_area.clone();
        Callback::from(move |pair: (i32, usize, usize, usize, usize)| {
            let mut discs = (*disciplines).clone();
            let selected_area = selected_area.clone();
            let disc = discs.iter_mut().find(|d| d.id == pair.4).unwrap();
            disc.set_interest(pair.0, pair.3, pair.2, pair.1);
            let comp_key = CompetencyInterest {
                discipline_id: pair.4,
                path_id: pair.3,
                area_id: pair.2,
                comp_id: pair.1,
                interest: pair.0,
            };

            // TODO Move to a method
            let competencies: Result<Vec<CompetencyInterest>, StorageError> =
                LocalStorage::get("competencies");
            match competencies {
                Ok(mut comps) => {
                    let comp = comps.iter_mut().position(|x| {
                        x.discipline_id == comp_key.discipline_id
                            && x.path_id == comp_key.path_id
                            && x.area_id == comp_key.area_id
                            && x.comp_id == comp_key.comp_id
                    });
                    match comp {
                        Some(c) => {
                            // TODO Duplicated logic right now
                            if comp_key.interest == comps[c].interest {
                                comps.remove(c);
                            } else {
                                comps[c].interest = comp_key.interest;
                            }
                            LocalStorage::set("competencies", comps).unwrap();
                        }
                        _ => {
                            comps.push(comp_key);
                            LocalStorage::set("competencies", comps).unwrap();
                        }
                    }
                }
                _ => {
                    LocalStorage::set("competencies", vec![comp_key]).unwrap();
                }
            }

            // TODO This is broken
            match (*selected_area).clone() {
                Some(area) => {
                    if area.id == comp_key.area_id {
                        let disc = disc.clone();
                        let area = disc
                            .paths
                            .into_iter()
                            .find_map(|p| p.areas.into_iter().find(|a| a.id == comp_key.area_id))
                            .unwrap();
                        selected_area.set(Some(area));
                    }
                }
                _ => {}
            }
            disciplines.set(discs);
        })
    };

    let on_area_selected_callback = {
        let selected_area = selected_area.clone();
        let disciplines = disciplines.clone();
        Callback::from(move |pair: (usize, usize, usize)| {
            let selected_area = selected_area.clone();
            let disc = disciplines.iter().find(|d| d.id == pair.2).unwrap(); // TODO Unwrap here
            let path = disc.paths.iter().find(|p| p.id == pair.1).unwrap();
            let area = path.areas.iter().find(|a| a.id == pair.0).unwrap();
            selected_area.set(Some(area.clone()));
        })
    };

    let graph = match (*selected_area).clone() {
        Some(area) => {
            let interest: Vec<i32> = area.competencies.iter().map(|c| c.interest).collect();
            let labels: Vec<String> = area
                .competencies
                .iter()
                .map(|c| c.name.to_string())
                .collect();
            html! {
                <Graph interest={interest} labels={labels}/>
            }
        }
        _ => {
            html! {
                <>
                </>
            }
        }
    };

    html! {
        <div class="discipline">
            <DisciplineList
                disciplines={(*disciplines).clone()}
                on_rating_changed={on_disc_rating_changed.clone()}
                on_area_selected={on_area_selected_callback.clone()}
            />
            {
                graph
            }
        </div>
    }
}
