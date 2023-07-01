use bevy::{prelude::*, render::camera::ScalingMode, asset::LoadState};

use crate::GameState;

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
	asset_server: Res<AssetServer>,
//	mouse: Res<Input<MouseButton>>,
	keyboard: Res<Input<KeyCode>>,
	disclaimer_query: Query<(Entity, &Handle<Image>), With<Disclaimer>>,
	lore_query: Query<Entity, (With<Lore>, Without<Disclaimer>)>,
	time: Res<Time>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	let mut flag = true;
	if keyboard.just_pressed(KeyCode::C) {
		for (disclaimer, texture) in disclaimer_query.iter() {
			// Check that the image actually loaded before progressing
			if asset_server.get_load_state(texture) == LoadState::Loaded && time.elapsed_seconds() > 10.0 {
				commands.entity(disclaimer).despawn_recursive();
			}
			flag = false;
		}
		if flag {
			for lore in lore_query.iter() {
				if time.elapsed_seconds() > 11.0 {
					commands.entity(lore).despawn_recursive();
					next_game_state.set(GameState::Game);
				}
			}
		}
	}
}

fn despawn_entities_with<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}
