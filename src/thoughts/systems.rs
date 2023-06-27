use bevy::prelude::*;
use crate::prelude::*;
use rand::Rng;

use super::data::ThoughtLibrary;

pub fn spawn_thoughts(
	mut commands: Commands,
	mut mesh: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	thoughts_entites: Query<Entity, With<Thought>>,
	player_transform: Query<&Transform, With<Player>>,
	asset_server: Res<AssetServer>,
	thoughts: Res<ThoughtLibrary>,
	spawn: Res<ThoughtSpawnParameters>,
) {
	let mut already_spawned = thoughts_entites.iter().count() as u32;

	let player_position = if let Ok(player) = player_transform.get_single() {
		player.translation
	} else {
		Vec3::ZERO
	};

	let mut spawning_capacity = 10;
	let mut rng = rand::thread_rng();
	while spawn.total_to_spawn > already_spawned && spawning_capacity > 0 {
		spawning_capacity -= 1;
		let thought_id: usize = rng.gen();
		let thought = thoughts.get_thought_by_index(thought_id % thoughts.n_thoughts());
		let location = player_position +
		if spawn.far_radius * spawn.far_radius > 2.0 * spawn.close_radius * spawn.close_radius {
			loop {
				let x = (2.0 * rng.gen::<f32>() - 1.0) * spawn.far_radius;
				let y = (2.0 * rng.gen::<f32>() - 1.0) * spawn.far_radius;
				let z = (2.0 * rng.gen::<f32>() - 1.0) * spawn.far_radius;
				let r2 = x*x+y*y+z*z;
				if r2 <= spawn.far_radius * spawn.far_radius && r2 >= spawn.close_radius * spawn.close_radius {
					break Vec3::new(x,y,z);
				}
			}
		} else {
			warn!("Spawning far radius and close radius are very close to each other, RNG might converge slowly");
			let x = (2.0 * rng.gen::<f32>() - 1.0) * spawn.far_radius;
			let y = (2.0 * rng.gen::<f32>() - 1.0) * spawn.far_radius;
			let z = (2.0 * rng.gen::<f32>() - 1.0) * spawn.far_radius;
			Vec3::new(x,y,z)
		};
		spawn_thought(&mut commands, &mut mesh, &mut materials, &asset_server, thought, location);
		already_spawned += 1;
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

pub fn collect_thoughts(
	mut commands: Commands,
	players: Query<Entity, With<Player>>,
	thoughts: Query<(Entity, &Thought), Without<Player>>,
	mut collisions: EventReader<PlayerInteractionEvent>,
	mut banana: EventWriter<ThoughtCollectedEvent>
) {
	for event in collisions.iter() {
		if let (Ok(player), Ok(thought)) = (players.get(event.player), thoughts.get(event.other)) {
			commands.entity(thought.0).despawn_recursive();
			banana.send(ThoughtCollectedEvent {
				player, thought: thought.1.clone()
			});
		}
	}
}

pub fn despawn_thoughts(
	mut commands: Commands,
	thoughts_entites: Query<(Entity, &Transform), With<Thought>>,
	player_transform: Query<&Transform, With<Player>>,
	despawn: Res<ThoughtSpawnParameters>,
) {
	if let Ok(player_transform) = player_transform.get_single() {
		for (entity, transform) in thoughts_entites.iter() {
			if (transform.translation - player_transform.translation).length() > despawn.despawn_radius {
				commands.entity(entity).despawn_recursive();
			}
		}
	}
}
