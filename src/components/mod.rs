use crate::models::*;
use crate::store::State;
use crate::{components::discipline_list::*, components::save_area::*};
use yew::prelude::*;
use yew_router::history::HistoryResult;
use yew_router::prelude::*;
use yewdux::prelude::Dispatch;

mod area_list;
mod competency_list;
mod discipline_list;
mod path_list;
mod rating_select;
mod save_area;

fn get_query_id(query: &HistoryResult<TreeId>) -> Option<String> {
    match query {
        Ok(query) => Some(query.id.clone()),
        _ => None,
    }
}

#[function_component(Competencies)]
pub fn competencies() -> Html {
    let dispatch = Dispatch::<State>::new();

    let location = use_location().unwrap();
    let query: HistoryResult<TreeId> = location.query();

    dispatch.reduce_mut(|state| state.refresh(get_query_id(&query)));

    html! {
        <div class="discipline">
            <SaveArea />
            <DisciplineList />
        </div>
    }
}
