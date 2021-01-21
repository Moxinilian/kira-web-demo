use crate::AppRoute;
use kira::{
    group::{handle::GroupHandle, GroupSet},
    manager::AudioManager,
    metronome::{handle::MetronomeHandle, MetronomeSettings},
    sound::{handle::SoundHandle, Sound, SoundSettings},
    Frame, Tempo,
};
use yew::prelude::*;
use yew_router::prelude::*;

pub struct DrumFillDemo {
    link: ComponentLink<Self>,

    loop_sound: Option<SoundHandle>,
    fill_2b: Option<SoundHandle>,
    fill_3b: Option<SoundHandle>,
    fill_4b: Option<SoundHandle>,

    manager: AudioManager,
    metronome: MetronomeHandle,
    group: GroupHandle,

    loaded: bool,
}

pub enum Message {
    LoadedLoop(u32, Vec<Frame>),
    LoadedFill2b(u32, Vec<Frame>),
    LoadedFill3b(u32, Vec<Frame>),
    LoadedFill4b(u32, Vec<Frame>),
}

impl Component for DrumFillDemo {
    type Message = Message;

    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link_clone = link.clone();
        crate::utils::load_audio_data("/drum-fill-demo/loop.ogg", move |rate, frames| {
            link_clone.send_message(Message::LoadedLoop(rate, frames))
        });
        let link_clone = link.clone();
        crate::utils::load_audio_data("/drum-fill-demo/2-beat-fill.ogg", move |rate, frames| {
            link_clone.send_message(Message::LoadedFill2b(rate, frames))
        });
        let link_clone = link.clone();
        crate::utils::load_audio_data("/drum-fill-demo/3-beat-fill.ogg", move |rate, frames| {
            link_clone.send_message(Message::LoadedFill3b(rate, frames))
        });
        let link_clone = link.clone();
        crate::utils::load_audio_data("/drum-fill-demo/4-beat-fill.ogg", move |rate, frames| {
            link_clone.send_message(Message::LoadedFill4b(rate, frames))
        });

        let mut manager = AudioManager::new(Default::default()).unwrap();
        let metronome = manager
            .add_metronome(MetronomeSettings::new().tempo(Tempo(128.0)))
            .unwrap();
        let group = manager.add_group(Default::default()).unwrap();

        Self {
            link,
            loop_sound: None,
            fill_2b: None,
            fill_3b: None,
            fill_4b: None,
            manager,
            metronome,
            group,
            loaded: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::LoadedLoop(rate, frames) => {
                self.loop_sound = self
                    .manager
                    .add_sound(Sound::from_frames(
                        rate,
                        frames,
                        SoundSettings::new().groups(GroupSet::new().add(&self.group)),
                    ))
                    .ok();
                self.check_loaded()
            }
            Message::LoadedFill2b(rate, frames) => {
                self.fill_2b = self
                    .manager
                    .add_sound(Sound::from_frames(
                        rate,
                        frames,
                        SoundSettings::new().groups(GroupSet::new().add(&self.group)),
                    ))
                    .ok();
                self.check_loaded()
            }
            Message::LoadedFill3b(rate, frames) => {
                self.fill_3b = self
                    .manager
                    .add_sound(Sound::from_frames(
                        rate,
                        frames,
                        SoundSettings::new().groups(GroupSet::new().add(&self.group)),
                    ))
                    .ok();
                self.check_loaded()
            }
            Message::LoadedFill4b(rate, frames) => {
                self.fill_4b = self
                    .manager
                    .add_sound(Sound::from_frames(
                        rate,
                        frames,
                        SoundSettings::new().groups(GroupSet::new().add(&self.group)),
                    ))
                    .ok();
                self.check_loaded()
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.loaded {
            html! {
                <>
                    <RouterButton<AppRoute> classes="small-button" route=AppRoute::Index>
                        { "Back" }
                    </RouterButton<AppRoute>>
                </>
            }
        } else {
            crate::utils::loading("drum fill demo")
        }
    }
}

impl DrumFillDemo {
    fn check_loaded(&mut self) -> ShouldRender {
        if self.loaded {
            return false;
        }

        if self.loop_sound.is_none()
            || self.fill_2b.is_none()
            || self.fill_3b.is_none()
            || self.fill_4b.is_none()
        {
            return false;
        }

        self.loaded = true;
        true
    }
}
