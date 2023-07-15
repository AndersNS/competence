use crate::bindings;
use gloo_console::log;
use serde::{Deserialize, Serialize};
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Dataset {
    label: String,
    backgroundColor: String,
    borderColor: String,
    data: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    labels: Vec<String>,
    datasets: Vec<Dataset>,
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    responsive: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    r#type: String,
    data: Data,
    options: Options,
}

#[derive(Clone, Properties, PartialEq)]
pub struct GraphProps {
    pub id: String,
    pub labels: Vec<String>,
    pub interest: Vec<i32>,
    pub competency: Vec<i32>,
}

fn draw_graph(interest: Vec<i32>, competency: Vec<i32>, labels: Vec<String>, graph_ref: &NodeRef) {
    let datasets = vec![
        Dataset {
            label: "Interest".to_string(),
            backgroundColor: "rgb(255, 99, 132, 0.5)".to_string(),
            borderColor: "rgb(255, 99, 132)".to_string(),
            data: interest,
        },
        Dataset {
            label: "Competency".to_string(),
            backgroundColor: "rgb(54, 162, 235, 0.5)".to_string(),
            borderColor: "rgb(54, 162, 235)".to_string(),
            data: competency,
        },
    ];
    let data = Data { labels, datasets };

    let js_value = serde_wasm_bindgen::to_value(&data).unwrap();
    bindings::renderChart(graph_ref.cast::<HtmlElement>().unwrap(), js_value);
}

#[function_component(Graph)]
pub fn graph(
    GraphProps {
        id,
        labels,
        interest,
        competency,
    }: &GraphProps,
) -> Html {
    let labels = labels.clone();
    let interest = interest.clone();
    let competency = competency.clone();

    let graph_ref = use_node_ref();
    {
        let interest1 = interest.clone();
        let labels1 = labels.clone();
        let competency1 = competency.clone();
        let id = id.clone();
        let graph_ref = graph_ref.clone();
        use_effect_with_deps(
            move |_| {
                log!("drawing graph");
                draw_graph(interest1, competency1, labels1, &graph_ref);
                || ()
            },
            (id, labels, interest, competency),
        );
    }

    html! {
        <>
            <div style="width:50rem">
              <canvas ref={graph_ref} id={id.to_string()}></canvas>
            </div>
        </>
    }
}
