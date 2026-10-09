use serde::{Deserialize, Serialize};
use tellus::{SchemaVersion, Versioned};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameGateEvent {}

impl Versioned for GameGateEvent {
    const MANIFEST: &'static str = "cribbage.game-gate-event";
    const VERSION: SchemaVersion = SchemaVersion::new(1);
}
