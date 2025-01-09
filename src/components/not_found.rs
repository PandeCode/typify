use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NotFoundProps {}

#[function_component]
pub fn NotFound(props: &NotFoundProps) -> Html {
    let NotFoundProps {} = props;
    html! { <div>{ "404 - Not found" }</div> }
}
