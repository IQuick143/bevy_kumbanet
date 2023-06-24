use bevy::prelude::*;
use rand::Rng;

use crate::prelude::{Thought, Player, Interactable};

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
	/*commands.spawn((MaterialMeshBundle {
		mesh: mesh.add(Mesh::from(shape::Quad::default())),
		material: materials.add(StandardMaterial {
			base_color_texture: Some(asset_server.load("ui/hand.png")),
			..default()
		}),
		..default()
		},
		Thought,
	))
	.insert(Name::new("Thought"));*/

	let mut rng = rand::thread_rng();
	for _ in 1..100 {
		let (x, y, z) = rng.gen();
		let thought = spawn_thought(&mut commands, &mut mesh, &mut materials, &asset_server, (Vec3::new(x, y, z) - 0.5) * 100.0);
		commands.entity(thought).insert(Name::new("Cringe"));
	}
}

fn spawn_thought(
	commands: &mut Commands,
	mesh: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	asset_server: &Res<AssetServer>,
	location: Vec3,
) -> Entity {
	commands.spawn((MaterialMeshBundle {
		mesh: mesh.add(Mesh::from(shape::Torus::default())),
		material: materials.add(StandardMaterial {
			base_color_texture: Some(asset_server.load("ui/hand.png")),
			..default()
		}),
		transform: Transform::from_translation(location),
		..default()
		},
		Thought,
		Interactable {radius: 1.0},
	))
	.id()
}

fn rotate_thoughts(
	mut thoughts_query: Query<&mut Transform, With<Thought>>,
	player_query: Query<&Transform, (With<Player>, Without<Thought>)>,
) {
	let player_transform = player_query.single();
	for mut thought_transform in thoughts_query.iter_mut() {
		thought_transform.look_at(-player_transform.translation, Vec3::Y);
	}
}