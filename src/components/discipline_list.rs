use crate::components::path_list::*;
use crate::export::export_tree;
use crate::models::*;
use crate::store::State;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Clone, Properties, PartialEq)]
pub struct DisciplineListProps {
    pub id: usize,
}

#[function_component(DisciplineList)]
pub fn discipline_list(DisciplineListProps { id }: &DisciplineListProps) -> Html {
    let (store, dispatch) = use_store::<State>();

    let on_disc_rating_changed = {
        Callback::from(move |rating: CompetencyRating| {
            dispatch.reduce_mut(|state| state.update_rating(rating));
        })
    };

    store
        .disciplines
        .iter()
        .find(|discipline| discipline.id == *id)
        .map_or_else(|| html! {
            <>
                <div class="d-flex">
                    <h1>{ "Not found" }</h1>
                </div>
            </>
            }, |disc| {
            let disc = disc.clone();
            let on_rating_changed = on_disc_rating_changed.clone();
            let on_disc_rating_changed = {
                Callback::from(move |mut rating: CompetencyRating| {
                    rating.discipline_id = disc.id;
                    on_rating_changed.emit(rating)
                })
            };

            let copy_as_csv_clicked = {
                let disc = disc.clone();
                Callback::from(move |e: MouseEvent| {
                    e.prevent_default();
                    let disc = disc.clone();
                    yew::platform::spawn_local(async move {
                        let csv_string = export_tree(&disc);
                        let window = web_sys::window().unwrap();
                        let promis = window.navigator().clipboard().unwrap().write_text(&csv_string);
                        let _result = wasm_bindgen_futures::JsFuture::from(promis).await.unwrap();
                    });
                })
            };

            html! {
            <>
                <div class="d-flex">
                    <h1>{disc.name}</h1>
                    <a class="csv-button" href="#" onclick={copy_as_csv_clicked.clone()}>{"Copy as csv"}</a>
                </div>
                <PathList
                    paths={disc.paths}
                    on_rating_changed={on_disc_rating_changed}
                />
            </>
            }
        })
}
