use crate::models::*;
use crate::{competency::discipline_list::*, routes::Route};
use gloo_console::log;
use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, BorrowMut};
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::*;

mod area_list;
mod competency_list;
mod discipline_list;
mod path_list;

fn update_local_storage(comp_rating: &CompetencyRating, id_suffix: &str) {
    let storage_id = storage_id(id_suffix);
    let competencies = get_competencies_from_localstorage(id_suffix);
    match competencies {
        Ok(mut comps) => {
            let comp = comps.iter_mut().position(|x| {
                x.discipline_id == comp_rating.discipline_id
                    && x.path_id == comp_rating.path_id
                    && x.area_id == comp_rating.area_id
                    && x.comp_id == comp_rating.comp_id
                    && x.rating == comp_rating.rating
            });
            match comp {
                Some(c) => {
                    if comp_rating.rating == Rating::Interest(0)
                        || comp_rating.rating == Rating::Competency(0)
                    {
                        comps.remove(c);
                    } else {
                        comps[c].rating = comp_rating.rating;
                    }

                    LocalStorage::set(storage_id, comps).unwrap();
                }
                _ => {
                    comps.push(*comp_rating);
                    LocalStorage::set(storage_id, comps).unwrap();
                }
            }
        }
        _ => {
            LocalStorage::set(storage_id, vec![comp_rating]).unwrap();
        }
    }
}

fn update_discs_from_ratings(fetched_discs: &mut Vec<Discipline>, ratings: &Vec<CompetencyRating>) {
    for ele in ratings.iter() {
        if let Some(index) = fetched_discs.iter().position(|c| c.id == ele.discipline_id) {
            fetched_discs[index].update_rating(ele.rating, ele.path_id, ele.area_id, ele.comp_id)
        }
    }
}

fn update_from_local_storage(fetched_discs: &mut Vec<Discipline>, id_suffix: &str) -> usize {
    let competencies = get_competencies_from_localstorage(id_suffix);
    match competencies {
        Ok(ratings) => {
            update_discs_from_ratings(fetched_discs, ratings.borrow());
            return ratings.len();
        }
        _ => {
            return 0;
        }
    }
}

fn storage_id(id_suffix: &str) -> String {
    format!("competencies{}", id_suffix)
}

fn get_competencies_from_localstorage(
    id_suffix: &str,
) -> Result<Vec<CompetencyRating>, StorageError> {
    let competencies: Result<Vec<CompetencyRating>, StorageError> =
        LocalStorage::get(storage_id(id_suffix));
    return competencies;
}

#[derive(Serialize, Deserialize, Clone)]
struct TreeId {
    id: String,
}

fn get_api_url() -> String {
    // "http://localhost:8787".to_string()
    "https://variantcompetency.anders-slinde.workers.dev".to_string()
}

#[function_component(Competencies)]
pub fn competencies() -> Html {
    let history = use_history().unwrap();
    let location = use_location().unwrap();
    let tree_id: UseStateHandle<Option<String>> = use_state(|| None);
    let unsaved_changes: UseStateHandle<bool> = use_state(|| false);
    let query: HistoryResult<TreeId> = location.query();

    let disciplines = use_state(|| vec![]);
    {
        let discs = disciplines.clone();
        let tree_id = tree_id.clone();
        let unsaved_changes = unsaved_changes.clone();
        use_effect_with_deps(
            move |_| {
                let discs = discs.clone();
                let unsaved_changes = unsaved_changes.clone();

                let query_id = match query {
                    Ok(id) => Some(id.id),
                    _ => None,
                };

                wasm_bindgen_futures::spawn_local(async move {
                    let mut fetched_discs: Vec<Discipline> = Request::get("/example.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    // Loop over stored values and update the state
                    let todo = query_id.clone();
                    match todo {
                        Some(val) => {
                            let request =
                                Request::get(&format!("{}/competency/{}", get_api_url(), val))
                                    .send()
                                    .await;
                            match request {
                                Ok(result) => {
                                    if result.status() == 200 {
                                        let ratings: Vec<CompetencyRating> =
                                            result.json().await.unwrap();

                                        for rating in ratings.iter() {
                                            update_local_storage(rating.borrow(), &val.borrow());
                                        }

                                        let updated = update_from_local_storage(
                                            fetched_discs.borrow_mut(),
                                            &val.borrow(),
                                        );
                                        let unsaved = updated - ratings.len();
                                        unsaved_changes.set(unsaved != 0);
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            update_from_local_storage(fetched_discs.borrow_mut(), "");
                        }
                    }

                    tree_id.set(query_id);
                    discs.set(fetched_discs);
                });
                || ()
            },
            location,
        );
    }

    let on_disc_rating_changed = {
        let disciplines = disciplines.clone();
        let unsaved_changes = unsaved_changes.clone();
        let tree_id = (*tree_id).clone();
        Callback::from(move |rating: CompetencyRating| {
            let mut discs = (*disciplines).clone();
            let tree_id = tree_id.clone();

            let disc = discs
                .iter_mut()
                .find(|d| d.id == rating.discipline_id)
                .unwrap();

            disc.update_rating(
                rating.rating,
                rating.path_id,
                rating.area_id,
                rating.comp_id,
            );

            match tree_id {
                Some(tree_id) => {
                    update_local_storage(&rating, &tree_id);
                }
                _ => {
                    update_local_storage(&rating, "");
                }
            }
            unsaved_changes.set(true);
            disciplines.set(discs);
        })
    };

    let save = {
        let history = history.clone();
        let unsaved_changes = unsaved_changes.clone();
        let current_id = (*tree_id).clone();
        Callback::from(move |_| {
            let history = history.clone();
            let unsaved_changes = unsaved_changes.clone();

            let tree_id = current_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let id = match tree_id {
                    Some(val) => TreeId { id: val },
                    None => TreeId {
                        id: Uuid::new_v4().as_simple().to_string(),
                    },
                };

                let competencies = get_competencies_from_localstorage(&id.id);
                let body = match competencies {
                    Ok(val) => {
                        for rating in val.iter() {
                            update_local_storage(&rating, &id.id);
                        }

                        serde_json::to_string(&val).unwrap()
                    }
                    _ => "[]".to_string(),
                };

                let request =
                    Request::post(&format!("{}/competency/{}", get_api_url(), id.id)).body(body);

                let response: Result<reqwasm::http::Response, reqwasm::Error> =
                    request.send().await;

                match response {
                    Ok(res) => {
                        let text = res.text().await.unwrap();
                        log!(format!("Got {} when posting competencies", text));
                        unsaved_changes.set(false);
                    }
                    _ => {}
                }
                history.push_with_query(Route::Home, id).unwrap();
            });
        })
    };

    html! {
        <div class="discipline">
            <p> {format!("Unsaved changes: {}", (*unsaved_changes).clone())} </p>
            <button onclick={save.clone()} > {"Save"} </button>
            <DisciplineList
                disciplines={(*disciplines).clone()}
                on_rating_changed={on_disc_rating_changed.clone()}
            />
        </div>
    }
}
