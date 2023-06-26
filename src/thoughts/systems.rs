use bevy::prelude::*;
use crate::prelude::*;
use rand::Rng;

use super::data::ThoughtLibrary;

pub fn spawn_thoughts(
	mut commands: Commands,
	mut mesh: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>,
	thoughts: Res<ThoughtLibrary>,
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
		let thought_id: usize = rng.gen();
		let thought = thoughts.get_thought_by_index(thought_id % thoughts.n_thoughts());

		let (x, y, z) = rng.gen();
		let _thought = spawn_thought(
			&mut commands, &mut mesh, &mut materials, &asset_server,
			thought, (Vec3::new(x, y, z) - 0.5) * 100.0
		);
	}
}

fn spawn_thought(
	commands: &mut Commands,
	mesh: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	asset_server: &Res<AssetServer>,
	thought: Thought,
	location: Vec3,
) -> Entity {
	commands.spawn((
		Name::new(format!("Thought {:?}@[{:.1};{:.1};{:.1}]", thought.word, location.x, location.y, location.z)),
		MaterialMeshBundle {
			mesh: mesh.add(Mesh::from(shape::Cube::default())),
			material: materials.add(thought.create_material(&asset_server)),
			transform: Transform::from_translation(location),
			..default()
		},
		thought,
		Interactable {radius: 1.0},
	))
	.id()
}

pub fn rotate_thoughts(
	mut thoughts_query: Query<&mut Transform, With<Thought>>,
	player_query: Query<&Transform, (With<Player>, Without<Thought>)>,
) {
	let player_transform = player_query.single();
	for mut thought_transform in thoughts_query.iter_mut() {
		thought_transform.look_at(-player_transform.translation, Vec3::Y);
	}
}
