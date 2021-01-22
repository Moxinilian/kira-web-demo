use yew::prelude::*;

use super::Beat;

#[derive(Debug, Properties, Clone, Copy)]
pub struct BeatDisplayProperties {
    pub beat: Option<Beat>,
}

pub struct BeatDisplay {
    beat: Option<Beat>,
}

impl Component for BeatDisplay {
    type Message = ();

    type Properties = BeatDisplayProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { beat: props.beat }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.beat = props.beat;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="beat-display">
                { (0..4).map(|i| html! {
                    <div
                        class="beat-display-tick"
                        filled=match self.beat {
                            Some(beat) => {
                                beat.as_usize() >= i
                            }
                            None => false,
                        }
                    />
                }).collect::<Html>() }
            </div>
        }
    }
}
