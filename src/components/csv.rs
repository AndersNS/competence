#[derive(Clone, Properties, PartialEq)]
pub struct CsvViewProps {}

#[function_component(CsvView)]
pub fn CsvView(CsvViewProps {}: &CsvViewProps) -> Html {
    html! {
        <>
        <h1>CSV!</h1>
        </>
    }
}
