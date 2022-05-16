use crate::competency::discipline_list::*;
use crate::models::*;
use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use reqwasm::http::Request;
use yew::prelude::*;

mod area_list;
mod competency_list;
mod discipline_list;
mod path_list;

#[function_component(Competencies)]
pub fn competencies() -> Html {
    let disciplines = use_state(|| vec![]);
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
                    let competencies: Result<Vec<CompetencyRating>, StorageError> =
                        LocalStorage::get("competencies");
                    match competencies {
                        Ok(comps) => {
                            for ele in comps {
                                if let Some(index) =
                                    fetched_discs.iter().position(|c| c.id == ele.discipline_id)
                                {
                                    fetched_discs[index].update_rating(
                                        ele.rating,
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
        Callback::from(move |pair: (Rating, usize, usize, usize, usize)| {
            let mut discs = (*disciplines).clone();
            let disc = discs.iter_mut().find(|d| d.id == pair.4).unwrap();
            disc.update_rating(pair.0, pair.3, pair.2, pair.1);
            let comp_rating = CompetencyRating {
                discipline_id: pair.4,
                path_id: pair.3,
                area_id: pair.2,
                comp_id: pair.1,
                rating: pair.0,
            };

            update_local_storage(comp_rating);

            disciplines.set(discs);
        })
    };

    html! {
        <div class="discipline">
            <DisciplineList
                disciplines={(*disciplines).clone()}
                on_rating_changed={on_disc_rating_changed.clone()}
            />
        </div>
    }
}

fn update_local_storage(comp_rating: CompetencyRating) {
    let competencies: Result<Vec<CompetencyRating>, StorageError> =
        LocalStorage::get("competencies");
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
                    if comp_rating.rating == comps[c].rating {
                        comps.remove(c);
                    } else {
                        comps[c].rating = comp_rating.rating;
                    }
                    LocalStorage::set("competencies", comps).unwrap();
                }
                _ => {
                    comps.push(comp_rating);
                    LocalStorage::set("competencies", comps).unwrap();
                }
            }
        }
        _ => {
            LocalStorage::set("competencies", vec![comp_rating]).unwrap();
        }
    }
}
