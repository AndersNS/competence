use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct RatingSelectProps {
    pub selected: Option<i32>,
    pub on_click: Callback<i32>,
    pub name: String,
}

#[function_component(RatingSelect)]
pub fn rating_select(
    RatingSelectProps {
        selected,
        on_click,
        name,
    }: &RatingSelectProps,
) -> Html {
    let on_click = on_click.clone();

    let range = (1..6)
        .map(|i| {
            let on_todo_select = {
                let on_click = on_click.clone();
                Callback::from(move |_| on_click.emit(i))
            };
            let class = if selected.is_some() && selected.unwrap() == i {
                "selected".to_string()
            } else {
                "".to_string()
            };
            html! {
                <p onclick={on_todo_select} class={classes!(class)}>{i}</p>
            }
        })
        .collect::<Html>();

    html! {
        <div class="rating">
            <p class="rating-name">{name}{":"}</p>
            <div class="rating-list">{range}</div>
        </div>
    }
}
