use bevy::prelude::*;

use crate::prelude::{Thought, Player};

pub struct ThoughtsPlugin;

impl Plugin for ThoughtsPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_startup_systems((
			spawn_thoughts,
		))
		.add_systems((
			rotate_thoughts,
		));
	}
}

fn spawn_thoughts(
	mut commands: Commands,
	mut mesh: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((MaterialMeshBundle {
		mesh: mesh.add(Mesh::from(shape::Quad::default())),
		material: materials.add(StandardMaterial {
			base_color_texture: Some(asset_server.load("ui/hand.png")),
			..default()
		}),
		..default()
		},
		Thought,
	))
	.insert(Name::new("Thought"));
}

fn rotate_thoughts(
	mut thoughts_query: Query<&mut Transform, With<Thought>>,
	player_query: Query<&Transform, (With<Player>, Without<Thought>)>,
) {
	let player_transform = player_query.single();
	//let player_translation = player_transform.translation;

	for mut thought_transform in thoughts_query.iter_mut() {
		/*let thought_forward = thought_transform.rotation * Vec3::Y;
		let to_player = (player_translation - thought_transform.translation).normalize();
		let forward_dot_player = thought_forward.dot(to_player);
		if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }
		let thought_right = thought_transform.rotation * Vec3::X;
		let right_dot_player = thought_right.dot(to_player);
		let rotation_sign = -f32::copysign(1.0, right_dot_player);
		let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos();
		let rotation_angle = rotation_sign * (5.0_f32).min(max_angle);

		thought_transform.rotate_z(rotation_angle);*/

		thought_transform.look_at(-player_transform.translation, Vec3::Y);
	}
}