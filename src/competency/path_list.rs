use crate::competency::area_list::*;
use crate::models::*;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct PathListProps {
    pub paths: Vec<Path>,
    pub on_rating_changed: Callback<(Rating, usize, usize, usize)>, // (new rating, competencyId, areaId)
}

#[function_component(PathList)]
pub fn path_list(
    PathListProps {
        paths,
        on_rating_changed,
    }: &PathListProps,
) -> Html {
    let on_rating_changed = on_rating_changed.clone();

    paths
        .iter()
        .map(|path| {
            let path = path.clone();
            let on_rating_changed = on_rating_changed.clone();
            let on_path_rating_changed = {
                Callback::from(move |pair: (Rating, usize, usize)| {
                    on_rating_changed.emit((pair.0, pair.1, pair.2, path.id))
                })
            };
            html! {
            <div class="path">
                <h2>{format!("{}", path.name)}</h2>
                <AreaList
                    sub_areas={path.areas.clone()}
                    on_rating_changed={on_path_rating_changed.clone()}
                    />
            </div>
            }
        })
        .collect()
}
