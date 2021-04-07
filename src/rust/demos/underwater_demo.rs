use crate::AppRoute;
use kira::{
    arrangement::{handle::ArrangementHandle, Arrangement, LoopArrangementSettings},
    instance::{InstanceSettings, StopInstanceSettings},
    manager::AudioManager,
    mixer::{
        effect::filter::{Filter, FilterSettings},
        SubTrackHandle,
    },
    parameter::{handle::ParameterHandle, tween::Tween, Mapping, ParameterSettings},
    sequence::{handle::SequenceInstanceHandle, Sequence},
    sound::{Sound, SoundSettings},
    Frame, Tempo, Value,
};
use yew::prelude::*;
use yew_router::prelude::*;

const EXPLANATION_TEXT: &str = "This demo uses a single \
parameter to control the cutoff frequency of a filter, \
the volume of the drums, and the volume of the pad.

Each of these values uses a different mapping to properly \
respond to the change in the \"underwater\" parameter.";

pub struct UnderwaterDemo {
    link: ComponentLink<Self>,

    bass: Option<ArrangementHandle>,
    pad: Option<ArrangementHandle>,
    lead: Option<ArrangementHandle>,
    drums: Option<ArrangementHandle>,

    manager: AudioManager,
    lead_track_handle: SubTrackHandle,
    underwater_parameter_handle: ParameterHandle,
    sequence_handle: Option<SequenceInstanceHandle<()>>,

    underwater: bool,
    loaded: bool,
}

pub enum Message {
    LoadedBass(u32, Vec<Frame>),
    LoadedPad(u32, Vec<Frame>),
    LoadedLead(u32, Vec<Frame>),
    LoadedDrums(u32, Vec<Frame>),

    PlayButtonClick,
    SubmergeButtonClick,
}

impl Component for UnderwaterDemo {
    type Message = Message;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Start loading all audio data
        let link_clone = link.clone();
        crate::utils::load_audio_data("/underwater-demo/bass.ogg", move |rate, frames| {
            link_clone.send_message(Message::LoadedBass(rate, frames))
        });
        let link_clone = link.clone();
        crate::utils::load_audio_data("/underwater-demo/pad.ogg", move |rate, frames| {
            link_clone.send_message(Message::LoadedPad(rate, frames))
        });
        let link_clone = link.clone();
        crate::utils::load_audio_data("/underwater-demo/lead.ogg", move |rate, frames| {
            link_clone.send_message(Message::LoadedLead(rate, frames))
        });
        let link_clone = link.clone();
        crate::utils::load_audio_data("/underwater-demo/drums.ogg", move |rate, frames| {
            link_clone.send_message(Message::LoadedDrums(rate, frames))
        });

        let mut manager = AudioManager::new(Default::default()).unwrap();
        let mut lead_track_handle = manager.add_sub_track(Default::default()).unwrap();
        let underwater_parameter_handle = manager
            .add_parameter(ParameterSettings::new().value(0.0))
            .unwrap();
        lead_track_handle
            .add_effect(
                Filter::new(FilterSettings::new().cutoff(Value::Parameter(
                    underwater_parameter_handle.id(),
                    Mapping {
                        input_range: (0.0, 1.0),
                        output_range: (8000.0, 2000.0),
                        ..Default::default()
                    },
                ))),
                Default::default(),
            )
            .unwrap();

        Self {
            link,
            bass: None,
            pad: None,
            lead: None,
            drums: None,
            manager,
            lead_track_handle,
            underwater_parameter_handle,
            sequence_handle: None,
            underwater: false,
            loaded: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::LoadedBass(rate, frames) => {
                self.bass = self
                    .manager
                    .add_sound(Sound::from_frames(
                        rate,
                        frames,
                        SoundSettings::new().semantic_duration(Tempo(85.0).beats_to_seconds(16.0)),
                    ))
                    .ok()
                    .and_then(|sound| {
                        self.manager
                            .add_arrangement(Arrangement::new_loop(&sound, Default::default()))
                            .ok()
                    });
                self.check_loaded()
            }
            Self::Message::LoadedPad(rate, frames) => {
                self.pad = self
                    .manager
                    .add_sound(Sound::from_frames(
                        rate,
                        frames,
                        SoundSettings::new().semantic_duration(Tempo(85.0).beats_to_seconds(16.0)),
                    ))
                    .ok()
                    .and_then(|sound| {
                        self.manager
                            .add_arrangement(Arrangement::new_loop(&sound, Default::default()))
                            .ok()
                    });
                self.check_loaded()
            }
            Self::Message::LoadedLead(rate, frames) => {
                self.lead = self
                    .manager
                    .add_sound(Sound::from_frames(
                        rate,
                        frames,
                        SoundSettings::new().semantic_duration(Tempo(85.0).beats_to_seconds(16.0)),
                    ))
                    .ok()
                    .and_then(|sound| {
                        self.manager
                            .add_arrangement(Arrangement::new_loop(
                                &sound,
                                LoopArrangementSettings::new()
                                    .default_track(self.lead_track_handle.id()),
                            ))
                            .ok()
                    });
                self.check_loaded()
            }
            Self::Message::LoadedDrums(rate, frames) => {
                self.drums = self
                    .manager
                    .add_sound(Sound::from_frames(
                        rate,
                        frames,
                        SoundSettings::new().semantic_duration(Tempo(85.0).beats_to_seconds(16.0)),
                    ))
                    .ok()
                    .and_then(|sound| {
                        self.manager
                            .add_arrangement(Arrangement::new_loop(&sound, Default::default()))
                            .ok()
                    });
                self.check_loaded()
            }
            Self::Message::PlayButtonClick => {
                if let Some(ref mut sequence_handle) = self.sequence_handle {
                    sequence_handle
                        .stop_sequence_and_instances(
                            StopInstanceSettings::new().fade_tween(Tween::linear(1.0)),
                        )
                        .ok();
                    self.sequence_handle = None;
                } else {
                    let sequence_handle = self
                        .manager
                        .start_sequence(
                            {
                                let mut sequence = Sequence::<()>::new(Default::default());
                                sequence.play(
                                    self.drums.as_ref().unwrap().id(),
                                    InstanceSettings::new().volume(Value::Parameter(
                                        self.underwater_parameter_handle.id(),
                                        Mapping {
                                            input_range: (0.0, 1.0),
                                            output_range: (1.0, 0.0),
                                            ..Default::default()
                                        },
                                    )),
                                );
                                sequence.play(self.bass.as_ref().unwrap().id(), Default::default());
                                sequence.play(
                                    self.pad.as_ref().unwrap().id(),
                                    InstanceSettings::new()
                                        .volume(self.underwater_parameter_handle.id()),
                                );
                                sequence.play(self.lead.as_ref().unwrap().id(), Default::default());
                                sequence
                            },
                            Default::default(),
                        )
                        .ok();
                    self.sequence_handle = sequence_handle;
                }

                true
            }
            Self::Message::SubmergeButtonClick => {
                if self.underwater {
                    self.underwater_parameter_handle
                        .set(0.0, Some(4.0.into()))
                        .ok();
                } else {
                    self.underwater_parameter_handle
                        .set(1.0, Some(4.0.into()))
                        .ok();
                }
                self.underwater = !self.underwater;
                true
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
                    <div class="container">
                        <div class="button-panel">
                            <button onclick=self.link.callback(|_| Self::Message::PlayButtonClick)>
                                {if self.sequence_handle.is_none() { "Play" } else { "Stop" }}
                            </button>
                            <button onclick=self.link.callback(|_| Self::Message::SubmergeButtonClick)>
                                {if self.underwater { "Resurface" } else { "Submerge" }}
                            </button>
                        </div>
                        <div class="explanation centered">
                            { EXPLANATION_TEXT }
                        </div>
                    </div>
                </>
            }
        } else {
            crate::utils::loading("underwater demo")
        }
    }
}

impl UnderwaterDemo {
    fn check_loaded(&mut self) -> ShouldRender {
        if self.loaded {
            return false;
        }

        if self.bass.is_none() || self.lead.is_none() || self.drums.is_none() || self.pad.is_none()
        {
            return false;
        }

        self.loaded = true;
        true
    }
}
