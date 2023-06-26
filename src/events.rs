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
