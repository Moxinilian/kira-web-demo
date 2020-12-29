#![recursion_limit="256"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod select_demo;
mod demos;
mod utils;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/underwater-demo"]
    UnderwaterDemo,
    #[to = "/"]
    Index,
}

struct Main;

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Index => html!{<select_demo::SelectDemo />},
                        AppRoute::UnderwaterDemo => html!{<demos::UnderwaterDemo />},
                    }
                })
            />
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Main>::new().mount_to_body();
}
