use crate::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct SelectDemo;

impl Component for SelectDemo {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="container">
                    <div class="title">{"Select a demo"}</div>
                </div>
                <RouterButton<AppRoute> classes="centered" route=AppRoute::UnderwaterDemo>
                    { "Underwater demo" }
                </RouterButton<AppRoute>>
            </>
        }
    }
}
