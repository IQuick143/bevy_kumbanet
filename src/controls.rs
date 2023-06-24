use bevy::{prelude::*, render::view::RenderLayers};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_startup_systems((
			spawn_hand,
		))
		.add_systems((

		))
		;
	}
}

fn spawn_hand(
	mut commands: Commands,
) {
	commands.spawn(SpriteBundle {
		transform: Transform::from_scale(Vec3::new(150.0, 150.0, 1.0)),
		..Default::default()
	})
	.insert(RenderLayers::layer(1))
	.insert(Name::new("Hand"));
}