use bevy::prelude::*;

use crate::prelude::Thought;

pub struct ThoughtCollectedEvent {
	pub player: Entity,
	pub thought: Thought
}

pub struct PlayerInteractionEvent {
	pub player: Entity,
	pub other: Entity
}

pub struct ChoreographyStopEvent {
	pub director: Entity,
}

pub struct ButtonPressEvent {
	pub button: Entity,
	pub button_type: ButtonType
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ButtonType {
	MergeThoughts
}
