use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct RatingProps {
    pub selected: Option<i32>,
    pub on_click: Callback<i32>,
    pub name: String,
}

#[function_component(Rating)]
pub fn rating(
    RatingProps {
        selected,
        on_click,
        name,
    }: &RatingProps,
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
