use yew::prelude::*;
use yew_router::prelude::*;

pub enum Message {}

pub struct DrumFillDemo {
    loaded: bool,
}

impl Component for DrumFillDemo {
    type Message = Message;

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { loaded: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        crate::utils::loading("drum fill demo")
    }
}
