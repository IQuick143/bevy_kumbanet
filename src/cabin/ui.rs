use bevy::{prelude::*, render::view::RenderLayers, math::Vec3Swizzles};

use crate::prelude::*;

use super::{CABIN_WIDTH, CABIN_HEIGHT};

pub fn spawn_ui(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((
		SpriteBundle {
			sprite: Sprite {custom_size: Some(Vec2::new(CABIN_WIDTH, CABIN_HEIGHT)), ..Default::default()},
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

	let button_size = Vec2::new(2.0, 2.0);
	commands.spawn((
		SpriteBundle {
			sprite: Sprite {custom_size: Some(button_size), ..Default::default()},
			texture: asset_server.load("thoughts/images/debug/-1.png"),
			transform: Transform::from_translation(Vec3::new(-7.0, -3.5, 0.0)),
			..Default::default()
		},
		RenderLayers::layer(1),
		Name::new("Merge Button"),
		CabinButton {
		    button: ButtonType::MergeThoughts,
		    half_extent: button_size / 2.0,
		},
	));
}

pub fn track_cursor(
	mut cursor_sprite: Query<&mut Transform, With<Hand>>,
	cursor: Res<CursorCabinPosition>,
) {
	for mut sprite in cursor_sprite.iter_mut() {
		sprite.translation = cursor.world_position.extend(sprite.translation.z);
	}
}

pub fn check_buttons(
	buttons: Query<(Entity, &Transform, &CabinButton)>,
	cursor: Res<CursorCabinPosition>,
	mouse: Res<Input<MouseButton>>,
	mut click_events: EventWriter<ButtonPressEvent>
) {
	if mouse.just_pressed(MouseButton::Left) {
		for (entity, transform, button) in buttons.iter() {
			let offset = (cursor.world_position - transform.translation.xy()).abs();
			if offset.x <= button.half_extent.x && offset.y <= button.half_extent.y {
				click_events.send(ButtonPressEvent {button: entity, button_type: button.button});
			}
		}
	}
}

pub fn spawn_bar(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((SpriteBundle{
		sprite: Sprite {custom_size: Some(Vec2::new(8.0, 1.0)), ..Default::default()},
		texture: asset_server.load("ui/bar_back.png"),
		transform: Transform::from_translation(Vec3::new(7.5, 0.0, 0.0))
			.with_rotation(Quat::from_rotation_z(3.14/2.0)),
		..Default::default()
	},
	RenderLayers::layer(1),
	Name::new("Bar"),
	));

	commands.spawn((SpriteBundle{
		sprite: Sprite {custom_size: Some(Vec2::new(4.5, 1.0)), ..Default::default()},
		texture: asset_server.load("ui/good_bar.png"),
		transform: Transform::from_translation(Vec3::new(7.5, 2.25, 1.0))
			.with_rotation(Quat::from_rotation_z(-3.14/2.0))
			.with_scale(Vec3::new(0.0, 1.0, 1.0)),
		..Default::default()
	},
	RenderLayers::layer(1),
	GoodBar,
	Name::new("Good Bar"),
	));

	commands.spawn((SpriteBundle{
		sprite: Sprite {custom_size: Some(Vec2::new(4.5, 1.0)), ..Default::default()},
		texture: asset_server.load("ui/bad_bar.png"),
		transform: Transform::from_translation(Vec3::new(7.5, -2.25, 1.0))
			.with_rotation(Quat::from_rotation_z(-3.14/2.0))
			.with_scale(Vec3::new(0.0, 1.0, 1.0)),
		..Default::default()
	},
	RenderLayers::layer(1),
	BadBar,
	Name::new("Bad Bar"),
	));
}

pub fn update_progress_bar(
	progress_bar: Res<ProgressBar>,
	mut good_query: Query<&mut Transform, With<GoodBar>>,
	mut bad_query: Query<&mut Transform, (With<BadBar>, Without<GoodBar>)>,
) {
	let mut good_transform = good_query.single_mut();
	let mut bad_transform = bad_query.single_mut();
	
	good_transform.scale.x = progress_bar.good_progress;
	bad_transform.scale.x = progress_bar.bad_progress;
}