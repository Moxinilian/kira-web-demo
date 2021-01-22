mod beat_display;

use crate::AppRoute;
use beat_display::BeatDisplay;
use kira::{
    group::{handle::GroupHandle, GroupSet},
    manager::AudioManager,
    metronome::{handle::MetronomeHandle, MetronomeSettings},
    sequence::{
        handle::SequenceInstanceHandle, Sequence, SequenceInstanceSettings, SequenceSettings,
    },
    sound::{handle::SoundHandle, Sound, SoundSettings},
    Duration, Frame, Tempo,
};
use yew::{
    prelude::*,
    services::{interval::IntervalTask, IntervalService},
};
use yew_router::prelude::*;

const EXPLANATION_TEXT: &str = "This demo uses \
a sequence to play a short drum sample repeatedly and \
keep track of which beat of music is currently playing. \
This beat is used to determine what kind of drum fill \
to play.

When the drum fill is triggered, a second sequence waits \
for the right beat, stops the previous sequence, starts \
the drum fill, and then starts a new loop.";

#[derive(Debug, Clone, Copy)]
pub enum DrumFill {
    TwoBeat,
    ThreeBeat,
    FourBeat,
}

impl DrumFill {
    fn length(self) -> usize {
        match self {
            DrumFill::TwoBeat => 2,
            DrumFill::ThreeBeat => 3,
            DrumFill::FourBeat => 4,
        }
    }

    fn start_interval(self) -> f64 {
        match self {
            DrumFill::FourBeat => 4.0,
            _ => 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Beat {
    One,
    Two,
    Three,
    Four,
}

impl Beat {
    fn as_usize(self) -> usize {
        match self {
            Beat::One => 0,
            Beat::Two => 1,
            Beat::Three => 2,
            Beat::Four => 3,
        }
    }

    fn fill(self) -> DrumFill {
        match self {
            Beat::One => DrumFill::ThreeBeat,
            Beat::Two => DrumFill::TwoBeat,
            _ => DrumFill::FourBeat,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrumFillEvent {
    Start,
    Finish,
}

#[derive(Debug, Clone, Copy)]
enum PlaybackState {
    Stopped,
    PlayingLoop(Beat),
    QueueingFill(Beat, DrumFill),
    PlayingFill(Beat, DrumFill),
}

impl PlaybackState {
    fn to_string(self) -> String {
        match self {
            PlaybackState::Stopped => "Stopped".into(),
            PlaybackState::PlayingLoop(_) => "Looping".into(),
            PlaybackState::QueueingFill(_, fill) => {
                format!("Queueing {}-beat drum fill", fill.length())
            }
            PlaybackState::PlayingFill(_, fill) => {
                format!("Playing {}-beat drum fill", fill.length())
            }
        }
    }

    fn current_beat(self) -> Option<Beat> {
        match self {
            PlaybackState::Stopped => None,
            PlaybackState::PlayingLoop(beat) => Some(beat),
            PlaybackState::QueueingFill(beat, _) | PlaybackState::PlayingFill(beat, _) => {
                Some(beat)
            }
        }
    }
}

pub struct DrumFillDemo {
    link: ComponentLink<Self>,

    loop_sound: Option<SoundHandle>,
    fill_2b: Option<SoundHandle>,
    fill_3b: Option<SoundHandle>,
    fill_4b: Option<SoundHandle>,

    manager: AudioManager,
    metronome: MetronomeHandle,
    group: GroupHandle,
    beat_tracker: Option<SequenceInstanceHandle<Beat>>,
    loop_sequence: Option<SequenceInstanceHandle<DrumFillEvent>>,

    loaded: bool,
    playback_state: PlaybackState,

    _interval_service: IntervalTask,
}

pub enum Message {
    LoadedLoop(u32, Vec<Frame>),
    LoadedFill2b(u32, Vec<Frame>),
    LoadedFill3b(u32, Vec<Frame>),
    LoadedFill4b(u32, Vec<Frame>),

    PlayClick,
    PlayFillClick,

    PopEvents,
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

        let interval_service = IntervalService::spawn(
            std::time::Duration::from_secs_f32(1.0 / 30.0),
            link.callback(|_| Message::PopEvents),
        );

        Self {
            link,
            loop_sound: None,
            fill_2b: None,
            fill_3b: None,
            fill_4b: None,
            manager,
            metronome,
            group,
            beat_tracker: None,
            loop_sequence: None,
            loaded: false,
            playback_state: PlaybackState::Stopped,
            _interval_service: interval_service,
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
            Message::PlayClick => {
                match self.playback_state {
                    PlaybackState::Stopped => {
                        self.playback_state = PlaybackState::PlayingLoop(Beat::One);
                        self.beat_tracker = Some(self.start_beat_tracker());
                        self.loop_sequence = Some(self.start_loop_sequence());
                        self.metronome.start().unwrap();
                    }
                    _ => {
                        self.group.stop(Default::default()).unwrap();
                        self.metronome.stop().unwrap();
                        self.playback_state = PlaybackState::Stopped;
                        self.beat_tracker = None;
                        self.loop_sequence = None;
                    }
                }
                true
            }
            Message::PlayFillClick => {
                match self.playback_state {
                    PlaybackState::PlayingLoop(beat) => {
                        let fill = beat.fill();
                        self.playback_state = PlaybackState::QueueingFill(beat, fill);
                        self.loop_sequence = Some(self.start_fill_and_loop_sequence(fill));
                    }
                    _ => {}
                }
                true
            }
            Message::PopEvents => {
                let mut should_render = false;
                if let Some(beat_tracker) = &mut self.beat_tracker {
                    while let Some(beat) = beat_tracker.pop_event() {
                        match &mut self.playback_state {
                            PlaybackState::Stopped => {}
                            PlaybackState::PlayingLoop(current_beat) => {
                                *current_beat = *beat;
                                should_render = true;
                            }
                            PlaybackState::QueueingFill(current_beat, _)
                            | PlaybackState::PlayingFill(current_beat, _) => {
                                *current_beat = *beat;
                                should_render = true;
                            }
                        }
                    }
                }
                if let Some(sequence) = &mut self.loop_sequence {
                    while let Some(event) = sequence.pop_event() {
                        match event {
                            DrumFillEvent::Start => {
                                if let PlaybackState::QueueingFill(beat, fill) = self.playback_state
                                {
                                    self.playback_state = PlaybackState::PlayingFill(beat, fill);
                                    should_render = true;
                                }
                            }
                            DrumFillEvent::Finish => {
                                if let PlaybackState::PlayingFill(beat, _) = self.playback_state {
                                    self.playback_state = PlaybackState::PlayingLoop(beat);
                                    should_render = true;
                                }
                            }
                        }
                    }
                }
                should_render
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
                            <button onclick=self.link.callback(|_| Self::Message::PlayClick)>
                                { match self.playback_state {
                                    PlaybackState::Stopped => "Play",
                                    _ => "Stop",
                                } }
                            </button>
                            <button onclick=self.link.callback(|_| Self::Message::PlayFillClick)>
                                { "Play fill" }
                            </button>
                        </div>
                        <BeatDisplay beat=self.playback_state.current_beat() />
                        <div class="playback-state-text">
                            { self.playback_state.to_string() }
                        </div>
                        <div class="explanation centered">
                            { EXPLANATION_TEXT }
                        </div>
                    </div>
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

    fn start_beat_tracker(&mut self) -> SequenceInstanceHandle<Beat> {
        self.manager
            .start_sequence(
                {
                    let mut sequence = Sequence::new(
                        SequenceSettings::new().groups(GroupSet::new().add(&self.group)),
                    );
                    sequence.wait_for_interval(1.0);
                    sequence.start_loop();
                    sequence.emit(Beat::One);
                    sequence.wait(Duration::Beats(1.0));
                    sequence.emit(Beat::Two);
                    sequence.wait(Duration::Beats(1.0));
                    sequence.emit(Beat::Three);
                    sequence.wait(Duration::Beats(1.0));
                    sequence.emit(Beat::Four);
                    sequence.wait(Duration::Beats(1.0));
                    sequence
                },
                SequenceInstanceSettings::new().metronome(&self.metronome),
            )
            .unwrap()
    }

    fn start_loop_sequence(&mut self) -> SequenceInstanceHandle<DrumFillEvent> {
        self.manager
            .start_sequence(
                {
                    let mut sequence = Sequence::new(
                        SequenceSettings::new().groups(GroupSet::new().add(&self.group)),
                    );
                    sequence.wait_for_interval(1.0);
                    sequence.start_loop();
                    sequence.play(self.loop_sound.as_ref().unwrap(), Default::default());
                    sequence.wait(Duration::Beats(4.0));
                    sequence
                },
                SequenceInstanceSettings::new().metronome(&self.metronome),
            )
            .unwrap()
    }

    fn start_fill_and_loop_sequence(
        &mut self,
        fill: DrumFill,
    ) -> SequenceInstanceHandle<DrumFillEvent> {
        let previous_loop_sequence = self.loop_sequence.take().unwrap();
        self.manager
            .start_sequence(
                {
                    let mut sequence = Sequence::new(
                        SequenceSettings::new().groups(GroupSet::new().add(&self.group)),
                    );
                    sequence.wait_for_interval(fill.start_interval());
                    sequence.emit(DrumFillEvent::Start);
                    sequence
                        .stop_sequence_and_instances(&previous_loop_sequence, Default::default());
                    sequence.play(
                        {
                            match fill {
                                DrumFill::TwoBeat => &self.fill_2b,
                                DrumFill::ThreeBeat => &self.fill_3b,
                                DrumFill::FourBeat => &self.fill_4b,
                            }
                            .as_ref()
                            .unwrap()
                        },
                        Default::default(),
                    );
                    sequence.wait_for_interval(4.0);
                    sequence.emit(DrumFillEvent::Finish);
                    sequence.start_loop();
                    sequence.play(self.loop_sound.as_ref().unwrap(), Default::default());
                    sequence.wait(Duration::Beats(4.0));
                    sequence
                },
                SequenceInstanceSettings::new().metronome(&self.metronome),
            )
            .unwrap()
    }
}
