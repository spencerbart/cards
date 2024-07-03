use std::sync::Arc;

use cards_proto::GameState;
use dashmap::DashMap;

pub mod services;

pub mod cards_proto {
    tonic::include_proto!("cards");
}

pub struct AppContext {
    pub game_states: Arc<DashMap<String, GameState>>,
}
