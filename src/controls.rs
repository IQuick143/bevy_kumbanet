use bevy::{prelude::*, render::{view::RenderLayers, camera::{Viewport, RenderTarget}}, core_pipeline::clear_color::ClearColorConfig};

use crate::{resources::MainRenderTexture, components::Hand};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_startup_systems((
			spawn_hand,
			spawn_hand_camera,
		))
		.add_systems((
			move_hand,
		))
		;
	}
}



fn spawn_hand(
	mut commands: Commands,
) {
	commands.spawn((SpriteBundle {
		transform: Transform::from_scale(Vec3::new(16.0, 16.0, 1.0)),
		..Default::default()
	},
	Hand
	))
	.insert(RenderLayers::layer(1))
	.insert(Name::new("Hand"));
}

fn spawn_hand_camera(
	mut commands: Commands,
	render_target: Res<MainRenderTexture>,
) {
	let size = UVec2::new(render_target.width/2, render_target.height/2);
	commands.spawn(Camera2dBundle {
		camera: Camera {
			viewport: Some(Viewport {physical_position: UVec2::new(size.x/2, size.y/2), physical_size: size, ..Default::default()}),
			target: RenderTarget::Image(render_target.texture.clone()),
			order: 10, is_active: true, ..Default::default()
		},
		camera_2d: Camera2d {clear_color: ClearColorConfig::None, ..Default::default()},
		..Default::default()
	})
	.insert(RenderLayers::layer(1))
	.insert(Name::new("Hand Camera"));
}

fn move_hand(
	window_query: Query<&Window>,
	mut hand_query: Query<&mut Transform, With<Hand>>,
) {
	let window = window_query.get_single().unwrap();
	let mut transform = hand_query.get_single_mut().unwrap();

	if let Some(current_pos) = window.cursor_position() {
		transform.translation.x = current_pos.x + 8.0 - window.width() / 2.0;
		transform.translation.y = current_pos.y - 8.0 - window.height() / 2.0;
	}
}