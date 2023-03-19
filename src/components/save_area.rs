use gloo_console::error;
use reqwasm::http::Request;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    local_storage::{get_competencies_from_localstorage, update_local_storage},
    models::TreeId,
    routes::Route,
    store::{get_api_url, State},
};

#[derive(Clone, Properties, PartialEq)]
pub struct SaveAreaProps {
    pub discipline: usize
}

#[function_component(SaveArea)]
pub fn save_area(SaveAreaProps { discipline }: &SaveAreaProps) -> Html {
    let (store, _dispatch) = use_store::<State>();
    let navigator = use_navigator().unwrap();

    let tree_id = store.tree_id.clone();
    let discipline_id = discipline.clone();

    let save_clicked = {
        let tree_id = tree_id.clone();
        let navigator = navigator.clone();
        let discipline_id = discipline_id.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let tree_id = tree_id.clone();
            let navigator = navigator.clone();
            let discipline_id = discipline_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let id = match tree_id.clone() {
                    Some(val) => val,
                    None => Uuid::new_v4().as_simple().to_string(),
                };

                let curr = tree_id.unwrap_or(String::from(""));

                let competencies = get_competencies_from_localstorage(&curr);

                let body = match competencies {
                    Ok(val) => {
                        for rating in val.iter() {
                            update_local_storage(&rating, &id);
                        }

                        serde_json::to_string(&val).unwrap()
                    }
                    _ => "[]".to_string(),
                };

                let request =
                    Request::post(&format!("{}/competency/{}", get_api_url(), id)).body(body);

                let response: Result<reqwasm::http::Response, reqwasm::Error> =
                    request.send().await;

                match response {
                    Ok(res) => {
                        if res.ok() {
                            navigator
                                .replace_with_query(&Route::Discipline { id: discipline_id.clone() }, &TreeId { id: id.clone() })
                                .unwrap();
                        } else {
                            error!("Saving failed!"); // TODO Better handling
                        }
                    }
                    _ => {}
                }
            });
        })
    };

    let unsaved = if store.unsaved_items {
        html! {
            <p>{"You have unsaved changes!"}</p>
        }
    } else {
        html! {
            <></>
        }
    };

    let save_text = if store.tree_id.is_some() {
        "Save changes"
    } else {
        "Open persistent shareable URL"
    };

    html! {
        <div class="save-area">
                {unsaved}
            <p>
                <a onclick={save_clicked.clone()} href="" > {save_text} </a>
            </p>
        </div>
    }
}
