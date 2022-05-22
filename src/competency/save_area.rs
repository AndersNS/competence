use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct SaveAreaProps {
    pub unsaved_changes: bool,
    pub on_save_clicked: Callback<bool>,
}

#[function_component(SaveArea)]
pub fn save_area(
    SaveAreaProps {
        unsaved_changes,
        on_save_clicked,
    }: &SaveAreaProps,
) -> Html {
    let on_save_clicked = on_save_clicked.clone();
    let save_clicked = { Callback::from(move |_| on_save_clicked.emit(true)) };

    html! {
        <div class="save-area">
                <p> {format!("Unsaved changes: {}", unsaved_changes)} </p>
                <button onclick={save_clicked.clone()} > {"Save"} </button>
        </div>
    }
}
