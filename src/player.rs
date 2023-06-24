use bevy::prelude::*;
use crate::{components::*, physics::PhysicsSystemSet};

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct PlayerHarness;

pub struct PlayerBehaviourPlugin;

impl Plugin for PlayerBehaviourPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system(player_controller)
		.add_system(player_transform.after(PhysicsSystemSet))
		;
	}
}

pub fn spawn_player(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
//	mut materials: ResMut<Assets<StandardMaterial>>
) {
	let capsule_handle = meshes.add(Mesh::from(shape::Capsule {
		radius: 0.5, depth: 1.0, ..Default::default()
	}));

	// Outer player transform object
	commands
	.spawn((Player,
		Velocity(Vec3::ZERO), VelocityDrag(0.1),
		AngularVelocity(Vec3::ZERO), AngularVelocityDrag(0.1),
		SpatialBundle::default()
	))
	.with_children(|player_holder| {
		// Visible player object
		player_holder
		.spawn(PbrBundle {
			mesh: capsule_handle,
			transform: Transform::default().looking_to(-Vec3::Y, Vec3::Z),
			..default()
		});
		// Unrotated transform
		player_holder.spawn((PlayerHarness, SpatialBundle::default()))
		.with_children(|_parent| {
			// Orthocams
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
		transform.rotation = Quat::IDENTITY;
	}
}
