use std::net::SocketAddr;

use gamercade_core::{Buttons, InputState};
use gamercade_sound_engine::SoundEngineData;
use ggrs::{Config, PlayerType};
use wasmtime::Global;

use super::WasmConsole;

#[derive(Clone)]
pub struct WasmConsoleState {
    pub(crate) previous_buttons: Box<[Buttons]>,
    pub(crate) memories: Vec<Vec<u8>>,
    pub(crate) mutable_globals: Vec<Global>,
    pub(crate) sound_engine_data: SoundEngineData,
}

pub struct SaveStateDefinition {
    pub(crate) memories: Vec<String>,
    pub(crate) mutable_globals: Vec<String>,
}

impl Config for WasmConsole {
    type Input = InputState;
    type State = WasmConsoleState;
    type Address = SocketAddr;
}

#[derive(Clone)]
pub struct SessionDescriptor {
    pub num_players: usize,
    pub player_types: Box<[PlayerType<SocketAddr>]>,
    pub port: u16,
}
