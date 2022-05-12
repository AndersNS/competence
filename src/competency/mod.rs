use crate::competency::discipline_list::*;
use crate::models::*;
use gloo_console::log;
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
    let on_disc_rating_changed = {
        Callback::from(move |pair: (u32, usize, usize, usize, usize)| {
            log!(format!(
                "New rating: {} for comp {} area: {} path: {} discipline: {}",
                pair.0, pair.1, pair.2, pair.3, pair.4
            ));
        })
    };
    html! {
        <div class="discipline">
            <DisciplineList disciplines={(*disciplines).clone()} on_rating_changed={on_disc_rating_changed.clone()} />
        </div>
    }
}
