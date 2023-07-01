use bevy::{prelude::*, render::view::RenderLayers, math::Vec3Swizzles};

use crate::prelude::*;

use super::{CABIN_WIDTH, CABIN_HEIGHT};

pub fn spawn_ui(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	// Hud
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

	// Cursor
	commands.spawn((
		SpriteBundle {
			sprite: Sprite {custom_size: Some(Vec2::new(0.5, 0.5)), ..Default::default()},
			texture: asset_server.load("thoughts/images/openclipart/screw_191883.png"),
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
			..Default::default()
		},
		RenderLayers::layer(1),
		Name::new("Hand"),
		Cursor,
	));

	// Merge button
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
	mut cursor_sprite: Query<&mut Transform, With<Cursor>>,
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

pub fn spawn_curtains(
	commands: &mut Commands,
	asset_server: Res<AssetServer>,
) -> (Entity, Entity) {
	(
		commands.spawn((SpriteBundle{
			sprite: Sprite {custom_size: Some(Vec2::new(CABIN_WIDTH * 0.5, CABIN_HEIGHT)), ..Default::default()},
			texture: asset_server.load("ui/curtain.png"),
			transform: Transform::from_translation(Vec3::new(-CABIN_WIDTH * 0.75, 0.0, 100.0))
			.with_scale(Vec3::new(-1.0, 1.0, 1.0)),
			..Default::default()
		},
		RenderLayers::layer(1),
		Name::new("Left Curtain"),
		)).id(),
		commands.spawn((SpriteBundle{
			sprite: Sprite {custom_size: Some(Vec2::new(CABIN_WIDTH * 0.5, CABIN_HEIGHT)), ..Default::default()},
			texture: asset_server.load("ui/curtain.png"),
			transform: Transform::from_translation(Vec3::new(CABIN_WIDTH * 0.75, 0.0, 100.0)),
			..Default::default()
		},
		RenderLayers::layer(1),
		Name::new("Right Curtain"),
		)).id()
	)
}

pub fn spawn_score_counter(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let score = 0;
	let font = asset_server.load("fonts/FiraSans-Bold.ttf");
	let text_style = TextStyle {
		font: font.clone(),
		font_size: 80.0,
		color: Color::rgba(0.9, 0.9, 0.1, 1.0),
	};
	let text_style_outline = TextStyle {
		font: font.clone(),
		font_size: 100.0,
		color: Color::rgba(0.1, 0.1, 0.1, 1.0),
	};	
	commands
		.spawn((Text2dBundle {
			text: Text::from_section(format!("{}", score), text_style)
				.with_alignment(TextAlignment::Center),
			transform: Transform::from_translation(Vec3::new(0.0, 3.8, 10.0))
				.with_scale(Vec3::splat(0.01)),
			..Default::default()
		},
		ScoreText(false),
		RenderLayers::layer(1),
	));
	commands
		.spawn((Text2dBundle {
			text: Text::from_section(format!("{}", score), text_style_outline)
				.with_alignment(TextAlignment::Center),
			transform: Transform::from_translation(Vec3::new(0.0, 3.8, 5.0))
				.with_scale(Vec3::splat(0.02)),
			..Default::default()
		},
		ScoreText(true),
		RenderLayers::layer(1),
	));
}

pub fn update_score_text(
	score_counter: Res<ScoreCounter>,
	mut score_query: Query<(&mut Transform, &mut Text, &ScoreText)>,
) {
	let score = score_counter.score;
	for (mut transform, mut score_text, outline) in score_query.iter_mut() {
		score_text.sections[0].value = format!("{}", score);
		let sin = (score_counter.timer.percent() * 3.14).sin();
		let cos = (score_counter.timer.percent() * 3.14).cos();
		if !outline.0 {
			transform.scale.x = 0.01 + sin * 0.005;
			transform.scale.y = 0.01 + cos * 0.005;
			score_text.sections[0].style.color = Color::rgba(
				score_text.sections[0].style.color.r() + sin,
				0.7,
				0.2 + score_counter.timer.percent() / 2.0,
				sin.abs());
		} else {
			transform.scale.x = 0.02 + cos * 0.01;
			transform.scale.y = 0.02 + sin * 0.005;
			score_text.sections[0].style.color = Color::rgba(
				score_text.sections[0].style.color.r() + cos,
				0.1,
				0.1,
				cos.abs());
		}
	}
}

/*
pub fn spawn_win_screen(
	mut commands: Commands,
	mut score_counter: ResMut<ScoreCounter>,
	asset_server: Res<AssetServer>,
) {
	if score_counter.score > 99999 {
		commands.spawn((SpriteBundle{
			sprite: Sprite {custom_size: Some(Vec2::new(16.0, 9.0)), ..Default::default()},
			texture: asset_server.load("boot/win.png"),
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
			..Default::default()
		},
		Win(Timer::from_seconds(0.1, TimerMode::Once)),
		RenderLayers::layer(1),
		Name::new("Win"),
		));
		score_counter.score = 0;
	}
}

pub fn despawn_win_screen(
	mut commands: Commands,
	mut win_query: Query<(Entity, &mut Win)>,
	time: Res<Time>,
) {
	for (entity, mut win) in win_query.iter_mut() {
		win.0.tick(time.delta());
		if win.0.just_finished() {
			commands
				.entity(entity).despawn_recursive();
		}
	}
}
*/

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