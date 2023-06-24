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
	asset_server: Res<AssetServer>,
) {
	commands.spawn((SpriteBundle {
		texture: asset_server.load("ui/hand.png"),
		transform: Transform::from_translation(Vec3::splat(1000.0)),
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
	let size = UVec2::new(render_target.width, render_target.height);
	commands.spawn(Camera2dBundle {
		camera: Camera {
			viewport: Some(Viewport {physical_position: UVec2::new(0, 0 /*size.x/2, size.y/2*/), physical_size: size, ..Default::default()}),
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
		let sprite_height = 32.0;
		let normalised_cursor_pos = Vec2::new(current_pos.x - window.width() / 2.0, current_pos.y - window.height() / 2.0);
		let root_pos = Vec2::new(0.0, -window.height() / 2.0);
		transform.translation.x = (normalised_cursor_pos.x + root_pos.x) / 2.0;
		transform.translation.y = (normalised_cursor_pos.y + root_pos.y) / 2.0;

		let direction = (normalised_cursor_pos - root_pos).normalize();

		transform.rotation = Quat::from_rotation_arc(Vec3::Y, direction.extend(0.0));
		transform.scale.y = normalised_cursor_pos.distance(root_pos) / sprite_height;
	}
}