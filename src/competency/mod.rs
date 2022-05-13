use crate::competency::discipline_list::*;
use crate::models::*;
use gloo_console::log;
use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use yew::prelude::*;

mod area_list;
mod competency_list;
mod discipline_list;
mod path_list;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
struct CompKey {
    discipline_id: usize,
    path_id: usize,
    area_id: usize,
    comp_id: usize,
    interest: u32,
}

impl fmt::Display for CompKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.discipline_id, self.path_id, self.area_id, self.comp_id, self.interest
        )
    }
}

impl FromStr for CompKey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let discipline_id = coords[0].parse::<usize>()?;
        let path_id = coords[1].parse::<usize>()?;
        let area_id = coords[2].parse::<usize>()?;
        let comp_id = coords[3].parse::<usize>()?;
        let interest = coords[3].parse::<u32>()?;

        Ok(CompKey {
            discipline_id,
            path_id,
            area_id,
            comp_id,
            interest,
        })
    }
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
                    let mut fetched_discs: Vec<Discipline> = Request::get("/example.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    // Loop over stored values and update the state
                    let competencies: Result<Vec<CompKey>, StorageError> =
                        LocalStorage::get("competencies");
                    match competencies {
                        Ok(comps) => {
                            for ele in comps {
                                log!(format!("{:?}", ele.discipline_id));
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

                    log!("Setting state");
                    discs.set(fetched_discs);
                });
                || ()
            },
            (),
        );
    }

    let d = (*disciplines).clone();

    let on_disc_rating_changed = {
        // TODO This is a huge mess
        // Refactor so we dont use a giant tuple
        // Maybe add an impementation to CompKey from tuple, or something else
        Callback::from(move |pair: (u32, usize, usize, usize, usize)| {
            let mut discs = (*disciplines).clone();
            let disc = discs.iter_mut().find(|d| d.id == pair.4).unwrap();
            disc.set_interest(pair.0, pair.3, pair.2, pair.1);
            let comp_key = CompKey {
                discipline_id: pair.4,
                path_id: pair.3,
                area_id: pair.2,
                comp_id: pair.1,
                interest: pair.0,
            };

            // TODO Move to a method
            let competencies: Result<Vec<CompKey>, StorageError> =
                LocalStorage::get("competencies");
            match competencies {
                Ok(mut comps) => {
                    let comp = comps.iter_mut().find(|x| {
                        x.discipline_id == comp_key.discipline_id
                            && x.path_id == comp_key.path_id
                            && x.area_id == comp_key.area_id
                            && x.comp_id == comp_key.comp_id
                    });
                    match comp {
                        Some(mut c) => {
                            c.interest = comp_key.interest;
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
            disciplines.set(discs);
        })
    };

    html! {
        <div class="discipline">
            <DisciplineList disciplines={d} on_rating_changed={on_disc_rating_changed.clone()} />
        </div>
    }
}
