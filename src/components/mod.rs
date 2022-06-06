use crate::local_storage::*;
use crate::models::*;
use crate::{components::discipline_list::*, components::save_area::*, routes::Route};
use gloo_console::error;
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
mod rating_select;
mod save_area;

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
                wasm_bindgen_futures::spawn_local(async move {
                    let mut fetched_discs: Vec<Discipline> = Request::get("/example.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    // Loop over stored values and update the state
                    match query.borrow() {
                        Ok(query_id) => {
                            let request = Request::get(&format!(
                                "{}/competency/{}",
                                get_api_url(),
                                query_id.id
                            ))
                            .send()
                            .await;
                            match request {
                                Ok(result) => {
                                    if result.status() == 200 {
                                        let ratings: Vec<CompetencyRating> =
                                            result.json().await.unwrap();

                                        let local_competencies =
                                            get_competencies_from_localstorage(&query_id.id);

                                        match local_competencies {
                                            Ok(local_ratings) => {
                                                if local_ratings.len() != ratings.len() {
                                                    unsaved_changes.set(true);
                                                }
                                            }
                                            _ => {}
                                        }
                                        for rating in ratings.iter() {
                                            update_local_storage(
                                                rating.borrow(),
                                                &query_id.id.borrow(),
                                            );
                                        }

                                        update_from_local_storage(
                                            fetched_discs.borrow_mut(),
                                            &query_id.id,
                                        );
                                    }
                                }
                                _ => {}
                            }
                            tree_id.set(Some(query_id.id.to_string()));
                        }
                        _ => {
                            update_from_local_storage(fetched_discs.borrow_mut(), "");
                            tree_id.set(None);
                        }
                    }

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
                        if res.ok() {
                            unsaved_changes.set(false);
                        } else {
                            error!("Saving failed!"); // TODO Better handling
                        }
                    }
                    _ => {}
                }
                history.push_with_query(Route::Home, id).unwrap();
            });
        })
    };
    let existing = (*tree_id).clone().is_none();
    html! {
        <div class="discipline">
            <SaveArea
                on_save_clicked={save.clone()}
                unsaved_changes={(*unsaved_changes).clone()}
                existing={!existing}
            />
            <DisciplineList
                disciplines={(*disciplines).clone()}
                on_rating_changed={on_disc_rating_changed.clone()}
            />
        </div>
    }
}
