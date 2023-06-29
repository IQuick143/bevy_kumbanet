use std::f32::consts::PI;

use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig, render::camera::{RenderTarget, Viewport}};
use crate::{prelude::*, physics::PhysicsSystemSet, GameState};

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct PlayerHarness;

pub struct PlayerBehaviourPlugin;

impl Plugin for PlayerBehaviourPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system(player_controller.run_if(in_state(GameState::Game)))
		.add_system(player_transform.after(PhysicsSystemSet).run_if(in_state(GameState::Game)))
		.add_system(player_boost.run_if(on_event::<ThoughtCutsceneEndEvent>()).run_if(in_state(GameState::Game)))
		;
	}
}

pub fn spawn_player_and_cameras(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	render_target: Res<MainRenderTexture>,
	asset_server: Res<AssetServer>,
) {
	/*let capsule_handle = meshes.add(Mesh::from(shape::Capsule {
		radius: 0.5, depth: 1.0, ..Default::default()
	}));*/
	let ship_handle = asset_server.load("ships/ship.glb#Scene0");

	let half_size = UVec2::new(render_target.width / 2, render_target.height / 2);

	// Outer player transform object
	commands
	.spawn((Player,
		render_target.texture.clone(),
		Velocity(Vec3::ZERO), VelocityDrag(0.1),
		AngularVelocity(Vec3::ZERO), AngularVelocityDrag(0.1),
		SpatialBundle::default()
	))
	.insert(Name::new("Player"))
	.with_children(|player_holder| {
		// Visible player object
		player_holder
		/*.spawn(PbrBundle {
			mesh: capsule_handle,
			transform: Transform::default().looking_to(-Vec3::Y, Vec3::Z),
			..default()
		})*/
		.spawn(SceneBundle {
			scene: ship_handle,
			transform: Transform::from_scale(Vec3::splat(0.5)),
			..Default::default()
		});
		// 3rd person camera
		player_holder
		.spawn(Camera3dBundle {
			camera: Camera {
				viewport: Some(Viewport {physical_position: UVec2::ZERO, physical_size: half_size, ..Default::default()}),
				target: RenderTarget::Image(render_target.texture.clone()),
				order: -1, is_active: true, ..Default::default()
			},
			projection: Projection::Perspective(PerspectiveProjection { fov: PI/3.0, aspect_ratio: 1.2, ..default() }),
			camera_3d: Camera3d {clear_color: ClearColorConfig::None, ..Default::default()},
			transform: Transform::from_translation(Vec3::new(0.0, 2.0, 8.0)).looking_at(Vec3::ZERO, Vec3::Y),
			..Default::default()
		})
		.insert(Name::new("3rd Person Camera"));
		// Unrotated transform
		player_holder.spawn((PlayerHarness, SpatialBundle::default())).insert(Name::new("Orthocams"))
		.with_children(|parent| {
			// Orthocams
			parent.spawn(Camera3dBundle {
				camera: Camera {
					viewport: Some(Viewport {physical_position: UVec2::new(half_size.x, 0), physical_size: half_size, ..Default::default()}),
					target: RenderTarget::Image(render_target.texture.clone()),
					order: -2, is_active: true, ..Default::default()
				},
				projection: Projection::Orthographic(OrthographicProjection {scale: 0.1, ..Default::default()}),
				camera_3d: Camera3d {clear_color: ClearColorConfig::None, ..Default::default()},
				transform: Transform::from_translation(10.0 * Vec3::Z).looking_at(Vec3::ZERO, Vec3::Y),
				..Default::default()
			})
			.insert(Name::new("Front View Camera"));
			parent.spawn(Camera3dBundle {
				camera: Camera {
					viewport: Some(Viewport {physical_position: half_size, physical_size: half_size, ..Default::default()}),
					target: RenderTarget::Image(render_target.texture.clone()),
					order: -3, is_active: true, ..Default::default()
				},
				projection: Projection::Orthographic(OrthographicProjection {scale: 0.1, ..Default::default()}),
				camera_3d: Camera3d {clear_color: ClearColorConfig::None, ..Default::default()},
				transform: Transform::from_translation(10.0 * Vec3::Y).looking_at(Vec3::ZERO, Vec3::X),
				..Default::default()
			})
			.insert(Name::new("Top Down View Camera"));
			parent.spawn((Camera3dBundle {
				camera: Camera {
					viewport: Some(Viewport {physical_position: UVec2::new(0, half_size.y), physical_size: half_size, ..Default::default()}),
					target: RenderTarget::Image(render_target.texture.clone()),
					order: -4, is_active: true, ..Default::default()
				},
				projection: Projection::Orthographic(OrthographicProjection {scale: 0.1, ..Default::default()}),
				camera_3d: Camera3d {clear_color: ClearColorConfig::None, ..Default::default()},
				transform: Transform::from_translation(10.0 * Vec3::X).looking_at(Vec3::ZERO, Vec3::Z),
				..Default::default()
			},
			ClearCamera(false)))
			.insert(Name::new("Side View Camera"));
		});
	});
}

pub fn player_controller(
	input: Res<Input<KeyCode>>,
	mut player: Query<(&Transform, &mut Velocity, &mut AngularVelocity), With<Player>>
) {
	for (transform, mut velocity, mut angular) in player.iter_mut() {
		if input.pressed(KeyCode::W) {
			velocity.0 += 0.01 * transform.forward();
		}
		if input.pressed(KeyCode::S) {
			velocity.0 -= 0.01 * transform.forward();
		}
		if input.pressed(KeyCode::D) {
			angular.0 -= 0.05 * transform.up();
		}
		if input.pressed(KeyCode::A) {
			angular.0 += 0.05 * transform.up();
		}
		if input.pressed(KeyCode::Q) {
			angular.0 -= 0.05 * transform.forward();
		}
		if input.pressed(KeyCode::E) {
			angular.0 += 0.05 * transform.forward();
		}
	}
}

pub fn player_transform(mut harness: Query<&mut Transform, With<PlayerHarness>>) {
	for mut transform in harness.iter_mut() {
		transform.look_to(Vec3::NEG_Z, Vec3::Y);
	}
}

fn player_boost(mut player: Query<(&Transform, &mut Velocity), With<Player>>, mut event: EventReader<ThoughtCutsceneEndEvent>) {
	let mut run = false;
	for _ in event.iter() {
		run = true;
	}
	if !run {
		return;
	}
	for (transform, mut velocity) in player.iter_mut() {
		velocity.0 += 500.0 * transform.forward()
	}
}

