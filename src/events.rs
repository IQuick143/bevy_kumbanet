use bevy::prelude::*;

pub struct PlayerInteractionEvent {
	pub player: Entity,
	pub other: Entity
}
