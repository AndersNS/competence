use crate::{bindings, models::Area};
use gloo_console::log;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;
use yew::prelude::*;

// TODO
// Take in props, disciplinelist
// Convert to data object
// Pass data object to the renderChart-function

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
    pub labels: Vec<String>,
    pub interest: Vec<i32>,
}

fn draw_graph(interest: Vec<i32>, labels: Vec<String>, graph_ref: &NodeRef) {
    let datasets = vec![Dataset {
        label: "Interest".to_string(),
        backgroundColor: "rgb(255, 99, 132, 0.5)".to_string(),
        borderColor: "rgb(255, 99, 132)".to_string(),
        data: interest,
    }];
    let data = Data { labels, datasets };

    let js_value = JsValue::from_serde(&data).unwrap();
    bindings::renderChart(graph_ref.cast::<HtmlElement>().unwrap(), js_value);
}

#[function_component(Graph)]
pub fn graph(GraphProps { labels, interest }: &GraphProps) -> Html {
    let labels = labels.clone();
    let interest = interest.clone();
    let graph_ref = use_node_ref();
    {
        let interest1 = interest.clone();
        let labels1 = labels.clone();
        let graph_ref = graph_ref.clone();
        use_effect_with_deps(
            move |_| {
                draw_graph(interest1, labels1, &graph_ref);
                || ()
            },
            (labels, interest),
        );
    }

    html! {
        <>
            <div style="width:40rem">
              <canvas ref={graph_ref} id="myChart"></canvas>
            </div>
        </>
    }
}
