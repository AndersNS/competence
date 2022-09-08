use crate::{
    local_storage::{
        get_competencies_from_localstorage, update_from_local_storage, update_local_storage,
    },
    models::{CompetencyRating, Discipline},
};
use gloo_console::error;
use reqwasm::http::Request;
use std::borrow::BorrowMut;
use yewdux::prelude::*;

// TODO Move this
pub fn get_api_url() -> String {
    // "http://localhost:8787".to_string()
    String::from("https://variantcompetency.anders-slinde.workers.dev")
}

#[derive(Clone, PartialEq, Default)]
pub enum Status {
    #[default]
    Ready,
    Loading,
    Error,
}

#[derive(Default, Clone, PartialEq, Store)]
pub struct State {
    pub disciplines: Vec<Discipline>,
    pub status: Status,
    pub is_loading: bool,
    pub unsaved_items: bool,
    pub tree_id: Option<String>,
}

impl State {
    pub fn update_rating(&mut self, rating: CompetencyRating) {
        self.unsaved_items = true;

        let disc = self
            .disciplines
            .iter_mut()
            .find(|d| d.id == rating.discipline_id)
            .unwrap();

        disc.update_rating(
            rating.rating,
            rating.path_id,
            rating.area_id,
            rating.comp_id,
        );

        match &self.tree_id {
            Some(tree_id) => {
                update_local_storage(&rating, &tree_id);
            }
            _ => {
                update_local_storage(&rating, "");
            }
        }
    }

    pub fn refresh(&mut self, query: Option<String>) {
        self.status = Status::Loading;

        yew::platform::spawn_local(async move {
            let dispatch = Dispatch::<State>::new();

            let mut unsaved_items = false;

            let mut fetched_discs: Vec<Discipline> = Request::get("/example.json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            match &query {
                Some(query_id) => {
                    let request =
                        Request::get(&format!("{}/competency/{}", get_api_url(), query_id))
                            .send()
                            .await;

                    match request {
                        Ok(result) => {
                            if result.status() == 200 {
                                let ratings: Vec<CompetencyRating> = result.json().await.unwrap();

                                let local_competencies =
                                    get_competencies_from_localstorage(&query_id);

                                match local_competencies {
                                    Ok(local_ratings) => {
                                        if local_ratings.len() != ratings.len() {
                                            unsaved_items = true;
                                        }
                                    }
                                    _ => {}
                                }

                                for rating in ratings.iter() {
                                    update_local_storage(rating, &query_id);
                                }

                                update_from_local_storage(fetched_discs.borrow_mut(), &query_id);
                            }
                        }
                        _ => {
                            error!("Error when getting data from backend");
                        }
                    }
                }
                _ => {
                    unsaved_items = true;
                    update_from_local_storage(fetched_discs.borrow_mut(), "");
                }
            }

            dispatch.reduce_mut(|state| {
                state.disciplines = fetched_discs;
                state.status = Status::Ready;
                state.unsaved_items = unsaved_items;
                state.tree_id = query;
            });
        });
    }
}
