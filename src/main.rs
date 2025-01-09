mod components;
mod router;
mod theme;
mod types;
mod utils;

use weblog::{
    console_error, // ,console_log
};
use yew::prelude::*;
use yew_router::prelude::*;

use components::nav_bar::NavBar;

#[function_component(App)]
fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    use_effect(|| {
        let res = theme::setup_theme();
        if res.is_err() {
            console_error!("Failed to load theme: ", res.err());
        }
    });

    html! {
        <HashRouter>
            <NavBar />
            <Switch<router::Route> render={router::switch} />
        </HashRouter>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
