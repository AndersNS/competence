use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct SaveAreaProps {
    pub unsaved_changes: bool,
    pub existing: bool,
    pub on_save_clicked: Callback<bool>,
}

#[function_component(SaveArea)]
pub fn save_area(
    SaveAreaProps {
        unsaved_changes,
        on_save_clicked,
        existing,
    }: &SaveAreaProps,
) -> Html {
    let on_save_clicked = on_save_clicked.clone();
    let save_clicked = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_save_clicked.emit(true);
        })
    };

    let unsaved = if *unsaved_changes {
        html! {
            <p>{"You have unsaved changes!"}</p>
        }
    } else {
        html! {
            <></>
        }
    };

    let save_text = if *existing {
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
