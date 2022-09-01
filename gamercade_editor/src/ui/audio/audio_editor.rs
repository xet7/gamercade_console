use std::{iter::Cycle, ops::Range, sync::Arc};

use eframe::egui::Ui;
use gamercade_audio::SFX_CHANNELS;
use gamercade_sound_engine::{
    SoundEngine, SoundEngineChannelType, SoundEngineData, SoundRomInstance,
};

use crate::editor_data::EditorSoundData;

use super::{
    ChainEditor, InstrumentEditor, Oscilloscope, OscilloscopeMode, PatternEditor, SfxEditor,
    SongEditor,
};

pub struct AudioEditor {
    pub mode: AudioEditorMode,
    chain_editor: ChainEditor,
    instrument_editor: InstrumentEditor,
    pattern_editor: PatternEditor,
    song_editor: SongEditor,
    sfx_editor: SfxEditor,

    sound_engine: SoundEngine,
    audio_sync_helper: AudioSyncHelper,

    oscilloscope: Oscilloscope,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AudioEditorMode {
    Instrument,
    Sfx,
    Songs,
    Chains,
    Patterns,
}

impl AudioEditor {
    pub(crate) fn new(data: &EditorSoundData) -> Self {
        let sound_rom_instance = Arc::new(SoundRomInstance::from(data));
        let mut sound_engine = SoundEngine::new(60, &sound_rom_instance, 64);

        let sound_engine_data =
            SoundEngineData::new(sound_engine.output_sample_rate(), &sound_rom_instance);

        let (producer, consumer) = rtrb::RingBuffer::new(sound_engine.output_sample_rate());

        sound_engine.send(SoundEngineChannelType::UpdateOutputProducer(Some(producer)));

        Self {
            mode: AudioEditorMode::Instrument,
            chain_editor: ChainEditor::default(),
            instrument_editor: InstrumentEditor::default(),
            pattern_editor: PatternEditor::default(),
            song_editor: SongEditor::default(),
            sfx_editor: SfxEditor::default(),
            sound_engine,
            audio_sync_helper: AudioSyncHelper {
                sync_rom: false,
                sound_engine_data,
                channel_ticker: (0..SFX_CHANNELS).cycle(),
                command_queue: Vec::new(),
            },
            oscilloscope: Oscilloscope::new(consumer),
        }
    }
}

pub(crate) enum AudioSyncCommand {
    PressedKey {
        note_index: usize,
        instrument_index: usize,
        channel: usize,
    },
    ReleasedKey {
        channel: usize,
    },
    TriggerNote {
        note_index: usize,
        instrument_index: usize,
    },
}

pub(crate) struct AudioSyncHelper {
    sync_rom: bool,
    pub(crate) sound_engine_data: SoundEngineData,
    channel_ticker: Cycle<Range<usize>>,
    command_queue: Vec<AudioSyncCommand>,
}

impl AudioSyncHelper {
    pub(crate) fn notify_rom_changed(&mut self) {
        self.sync_rom = true;
    }

    pub(crate) fn play_note(&mut self, note_index: usize, instrument_index: usize) -> usize {
        let channel = self.channel_ticker.next().unwrap();
        self.command_queue.push(AudioSyncCommand::PressedKey {
            note_index,
            instrument_index,
            channel,
        });
        channel
    }

    pub(crate) fn stop_note(&mut self, channel: usize) {
        self.command_queue
            .push(AudioSyncCommand::ReleasedKey { channel })
    }

    pub(crate) fn trigger_note(&mut self, note_index: usize, instrument_index: usize) {
        self.command_queue.push(AudioSyncCommand::TriggerNote {
            note_index,
            instrument_index,
        })
    }

    fn push_commands(&mut self, engine: &mut SoundEngine, data: &EditorSoundData) {
        if self.sync_rom {
            self.sync_rom = false;

            let new_instance = Arc::new(SoundRomInstance::from(data));
            self.sound_engine_data
                .replace_sound_rom_instance(&new_instance);
            engine.send(SoundEngineChannelType::SoundRomInstance(new_instance));
        }

        self.command_queue
            .drain(..)
            .for_each(|command| match command {
                AudioSyncCommand::PressedKey {
                    note_index,
                    instrument_index,
                    channel,
                } => engine.send(SoundEngineChannelType::PianoKeyPressed {
                    note_index,
                    instrument_index,
                    channel,
                }),
                AudioSyncCommand::ReleasedKey { channel } => {
                    engine.send(SoundEngineChannelType::PianoKeyReleased { channel })
                }
                AudioSyncCommand::TriggerNote {
                    note_index,
                    instrument_index,
                } => engine.send(SoundEngineChannelType::TriggerNote {
                    note_index,
                    instrument_index,
                    channel: self.channel_ticker.next().unwrap(),
                }),
            });
    }
}

impl AudioEditor {
    pub fn draw_selector(&mut self, ui: &mut Ui) {
        ui.selectable_value(&mut self.mode, AudioEditorMode::Instrument, "Instruments");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Patterns, "Patterns");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Chains, "Chains");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Songs, "Songs");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Sfx, "Sfx");

        ui.separator();

        ui.label("Oscilloscope:");
        if ui
            .selectable_value(&mut self.oscilloscope.mode, OscilloscopeMode::Off, "Off")
            .clicked()
        {
            self.oscilloscope.open = false;
        };
        // TODO: Add this back in when we have per-channel oscilloscope
        // if ui
        //     .selectable_value(
        //         &mut self.oscilloscope.mode,
        //         OscilloscopeMode::Channels,
        //         "Channels",
        //     )
        //     .clicked()
        // {
        //     self.oscilloscope.open = true
        // };
        if ui
            .selectable_value(
                &mut self.oscilloscope.mode,
                OscilloscopeMode::Master,
                "Master",
            )
            .clicked()
        {
            self.oscilloscope.open = true
        };

        self.oscilloscope.draw(ui);
    }

    pub fn draw_contents(&mut self, ui: &mut Ui, data: &mut EditorSoundData) {
        match self.mode {
            AudioEditorMode::Instrument => {
                self.instrument_editor
                    .draw(ui, data, &mut self.audio_sync_helper)
            }
            AudioEditorMode::Sfx => self.sfx_editor.draw(ui, data, &mut self.audio_sync_helper),
            AudioEditorMode::Songs => self.song_editor.draw(ui, data, &mut self.audio_sync_helper),
            AudioEditorMode::Chains => {
                self.chain_editor
                    .draw(ui, data, &mut self.audio_sync_helper)
            }
            AudioEditorMode::Patterns => {
                self.pattern_editor
                    .draw(ui, data, &mut self.audio_sync_helper)
            }
        };

        self.audio_sync_helper
            .push_commands(&mut self.sound_engine, data);
    }

    pub fn draw_bottom_panel(&mut self, ui: &mut Ui) {
        //TODO: Write this
        ui.label("TODO!");
    }
}
