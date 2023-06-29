use bevy::{prelude::*, render::camera::{RenderTarget, ScalingMode}};

use crate::{GameState, prelude::MainRenderTexture};

pub struct BootPlugin;

impl Plugin for BootPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_systems((
			initial_setup,
		).in_schedule(OnEnter(GameState::Boot)))
		.add_systems((
			advance_slides.run_if(in_state(GameState::Boot)),
		))
		.add_systems((
			despawn_entities_with::<Boot>,
		).in_schedule(OnExit(GameState::Boot)))
		;
	}
}

#[derive(Component)]
struct Boot;

#[derive(Component)]
struct Disclaimer;

#[derive(Component)]
struct Lore;

fn initial_setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands
		.spawn((
			Camera2dBundle {
				projection: OrthographicProjection {scaling_mode: ScalingMode::Fixed{width: 16.0, height: 9.0}, ..Default::default()},
				transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0)),
				..Default::default()
			},
			Boot,
			Name::new("Boot Camera"),
	));

	commands.spawn((
		SpriteBundle {
			sprite: Sprite {custom_size: Some(Vec2::new(16.0, 9.0)), ..Default::default()},
			texture: asset_server.load("boot/disclaimer.png"),
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
			..Default::default()
		},
		Boot,
		Disclaimer,
		Name::new("Disclaimer"),
	));

	commands.spawn((
		SpriteBundle {
			sprite: Sprite {custom_size: Some(Vec2::new(16.0, 9.0)), ..Default::default()},
			texture: asset_server.load("boot/lore.png"),
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 50.0)),
			..Default::default()
		},
		Boot,
		Lore,
		Name::new("Lore"),
	));
}

fn advance_slides(
	mut commands: Commands,
	mouse: Res<Input<MouseButton>>,
	keyboard: Res<Input<KeyCode>>,
	disclaimer_query: Query<Entity, With<Disclaimer>>,
	lore_query: Query<Entity, (With<Lore>, Without<Disclaimer>)>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	let mut flag = true;
	if mouse.just_pressed(MouseButton::Left) || keyboard.just_pressed(KeyCode::Return) {
		for disclaimer in disclaimer_query.iter() {
			commands
				.entity(disclaimer).despawn_recursive();
			flag = false;
		}
		if flag {
			for lore in lore_query.iter() {
				commands
					.entity(lore).despawn_recursive();
				next_game_state.set(GameState::Game);
			}
		}
	}
}

fn despawn_entities_with<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}