use bevy::prelude::*;

pub enum InputEvent {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    Stop,
}

pub enum EventStage {
    Started,
    Stopped,
}
impl EventStage {
    pub fn from_started(started: bool) -> Self {
        match started {
            true => EventStage::Started,
            false => EventStage::Stopped,
        }
    }
}
pub struct BurnBurnableEvent {
    pub burn_entity: Entity,
    pub burnable_entity: Entity,
    pub stage: EventStage,
}
