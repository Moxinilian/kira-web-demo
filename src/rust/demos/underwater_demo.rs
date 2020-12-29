use crate::AppRoute;
use kira::{
    arrangement::{Arrangement, ArrangementId, LoopArrangementSettings},
    instance::{InstanceSettings, StopInstanceSettings},
    manager::AudioManager,
    mixer::{
        effect::filter::{Filter, FilterSettings},
        SubTrackId,
    },
    parameter::{Mapping, ParameterId, Tween},
    playable::PlayableSettings,
    sequence::{Sequence, SequenceInstanceId},
    sound::Sound,
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

    bass: Option<ArrangementId>,
    pad: Option<ArrangementId>,
    lead: Option<ArrangementId>,
    drums: Option<ArrangementId>,

    manager: AudioManager,
    lead_track_id: SubTrackId,
    underwater_parameter_id: ParameterId,
    sequence_id: Option<SequenceInstanceId>,

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
        let lead_track_id = manager.add_sub_track(Default::default()).unwrap();
        let underwater_parameter_id = manager.add_parameter(0.0).unwrap();
        manager
            .add_effect_to_track(
                lead_track_id,
                Filter::new(FilterSettings::new().cutoff(Value::Parameter(
                    underwater_parameter_id,
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
            lead_track_id,
            underwater_parameter_id,
            sequence_id: None,
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
                        PlayableSettings::new()
                            .semantic_duration(Tempo(85.0).beats_to_seconds(16.0)),
                    ))
                    .and_then(|x| {
                        self.manager
                            .add_arrangement(Arrangement::new_loop(x, Default::default()))
                    })
                    .ok();
                self.check_loaded()
            }
            Self::Message::LoadedPad(rate, frames) => {
                self.pad = self
                    .manager
                    .add_sound(Sound::from_frames(
                        rate,
                        frames,
                        PlayableSettings::new()
                            .semantic_duration(Tempo(85.0).beats_to_seconds(16.0)),
                    ))
                    .and_then(|x| {
                        self.manager
                            .add_arrangement(Arrangement::new_loop(x, Default::default()))
                    })
                    .ok();
                self.check_loaded()
            }
            Self::Message::LoadedLead(rate, frames) => {
                self.lead = self
                    .manager
                    .add_sound(Sound::from_frames(
                        rate,
                        frames,
                        PlayableSettings::new()
                            .semantic_duration(Tempo(85.0).beats_to_seconds(16.0)),
                    ))
                    .and_then(|x| {
                        self.manager.add_arrangement(Arrangement::new_loop(
                            x,
                            LoopArrangementSettings::new().default_track(self.lead_track_id),
                        ))
                    })
                    .ok();
                self.check_loaded()
            }
            Self::Message::LoadedDrums(rate, frames) => {
                self.drums = self
                    .manager
                    .add_sound(Sound::from_frames(
                        rate,
                        frames,
                        PlayableSettings::new()
                            .semantic_duration(Tempo(85.0).beats_to_seconds(16.0)),
                    ))
                    .and_then(|x| {
                        self.manager
                            .add_arrangement(Arrangement::new_loop(x, Default::default()))
                    })
                    .ok();
                self.check_loaded()
            }
            Self::Message::PlayButtonClick => {
                if let Some(sequence_id) = self.sequence_id {
                    self.manager
                        .stop_sequence_and_instances(
                            sequence_id,
                            StopInstanceSettings::new().fade_tween(Tween::linear(1.0)),
                        )
                        .ok();
                    self.sequence_id = None;
                } else {
                    let sequence_id = self
                        .manager
                        .start_sequence(
                            {
                                let mut sequence = Sequence::<()>::new(Default::default());
                                sequence.play(
                                    self.drums.unwrap(),
                                    InstanceSettings::new().volume(Value::Parameter(
                                        self.underwater_parameter_id,
                                        Mapping {
                                            input_range: (0.0, 1.0),
                                            output_range: (1.0, 0.0),
                                            ..Default::default()
                                        },
                                    )),
                                );
                                sequence.play(self.bass.unwrap(), Default::default());
                                sequence.play(
                                    self.pad.unwrap(),
                                    InstanceSettings::new().volume(self.underwater_parameter_id),
                                );
                                sequence.play(self.lead.unwrap(), Default::default());
                                sequence
                            },
                            Default::default(),
                        )
                        .ok()
                        .map(|(id, _)| id);
                    self.sequence_id = sequence_id;
                }

                true
            }
            Self::Message::SubmergeButtonClick => {
                if self.underwater {
                    self.manager
                        .set_parameter(self.underwater_parameter_id, 0.0, Some(4.0.into()))
                        .ok();
                } else {
                    self.manager
                        .set_parameter(self.underwater_parameter_id, 1.0, Some(4.0.into()))
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
                                {if self.sequence_id.is_none() { "Play" } else { "Stop" }}
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
