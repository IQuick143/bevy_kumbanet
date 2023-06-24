use bevy::prelude::*;
use crate::components::*;

#[derive(SystemSet, Hash, Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct PhysicsSystemSet;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_systems((velocity_integration, rotation_integration, velocity_drag, angular_velocity_drag).chain().in_set(PhysicsSystemSet))
		;
	}
}

fn velocity_integration(
	mut entities: Query<(&Velocity, &mut Transform)>,
	time: Res<Time>
) {
	let dt = time.delta_seconds();
	for (v, mut transform) in entities.iter_mut() {
		transform.translation += v.0 * dt;
	}
}

fn velocity_drag(
	mut entities: Query<(&mut Velocity, &VelocityDrag)>,
	time: Res<Time>
) {
	let dt = time.delta_seconds();
	for (mut v, d) in entities.iter_mut() {
		v.0 *= 1.0 - d.0 * dt;
	}
}

fn rotation_integration(
	mut entities: Query<(&AngularVelocity, &mut Transform)>,
	time: Res<Time>
) {
	let dt = time.delta_seconds();
	for (v, mut transform) in entities.iter_mut() {
		let angle = v.length() * dt;
		if v.length() > 0.00001 {
			transform.rotate_axis(v.normalize(), angle);
		}
	}
}

fn angular_velocity_drag(
	mut entities: Query<(&mut AngularVelocity, &AngularVelocityDrag)>,
	time: Res<Time>
) {
	let dt = time.delta_seconds();
	for (mut v, d) in entities.iter_mut() {
		v.0 *= 1.0 - d.0 * dt;
	}
}