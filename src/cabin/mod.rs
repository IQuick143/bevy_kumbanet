use bevy::{prelude::*, render::{view::RenderLayers, camera::{RenderTarget, ScalingMode}}, core_pipeline::clear_color::ClearColorConfig, input::mouse::MouseMotion};

use crate::prelude::*;

pub struct CabinPlugin;

impl Plugin for CabinPlugin {
	fn build(&self, app: &mut App) {
		app
		.init_resource::<CursorCabinPosition>()
		.add_startup_systems((
			spawn_cabin_camera,
			spawn_cabin,
		))
		.add_systems((
			update_cursor_position,
			track_cursor,
		).chain())
		;
	}
}

fn spawn_cabin_camera(
	mut commands: Commands,
	render_target: Res<MainRenderTexture>,
) {
	commands.spawn((
		Camera2dBundle {
			camera: Camera {
				//viewport: Some(Viewport {physical_position: UVec2::new(0, 0 /*size.x/2, size.y/2*/), physical_size: size, ..Default::default()}),
				target: RenderTarget::Image(render_target.texture.clone()),
				order: 10, is_active: true, ..Default::default()
			},
			projection: OrthographicProjection {scaling_mode: ScalingMode::Fixed{width: 16.0, height: 9.0}, ..Default::default()},
			camera_2d: Camera2d {clear_color: ClearColorConfig::None},
			..Default::default()
		},
		CabinCamera,
		RenderLayers::layer(1),
		Name::new("Cabin Camera"),
	));
}

fn spawn_cabin(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((
		SpriteBundle {
			sprite: Sprite {custom_size: Some(Vec2::new(16.0, 9.0)), ..Default::default()},
			texture: asset_server.load("ui/hud.png"),
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
			..Default::default()
		},
		RenderLayers::layer(1),
		Name::new("HUD"),
	));

	commands.spawn((
		SpriteBundle {
			sprite: Sprite {custom_size: Some(Vec2::new(0.5, 0.5)), ..Default::default()},
			texture: asset_server.load("thoughts/images/debug/2.png"),
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
			..Default::default()
		},
		RenderLayers::layer(1),
		Name::new("Hand"),
		Hand,
	));
}

fn track_cursor(
	mut cursor_sprite: Query<&mut Transform, With<Hand>>,
	cursor: Res<CursorCabinPosition>,
) {
	for mut sprite in cursor_sprite.iter_mut() {
		sprite.translation = cursor.world_position.extend(sprite.translation.z);
	}
}

fn update_cursor_position(
	window_query: Query<&Window>,
	mut cursor: ResMut<CursorCabinPosition>,
) {
	let window = window_query.get_single().expect("There should be a single window");
	if let Some(pos) = window.cursor_position() {
		cursor.uv_position = Vec2::new(
			pos.x / window.width(),
			pos.y / window.height(),
		);
		cursor.world_position = Vec2::new(
			(cursor.uv_position.x - 0.5) * 16.0,
			(cursor.uv_position.y - 0.5) *  9.0,
		);
	}
}
