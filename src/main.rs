use bevy::prelude::*;

mod post_processing;
mod components;
mod test_scene;
mod physics;
mod player;

fn main() {
	let default_plugins = DefaultPlugins;
	
	#[cfg(debug_assertions)]
	let default_plugins = default_plugins.set(AssetPlugin {
		watch_for_changes: true,
		..Default::default()
	});

	App::new()
	.add_plugins(default_plugins)
	.add_plugin(physics::PhysicsPlugin {})
	.add_plugin(player::PlayerBehaviourPlugin {})
	.add_plugin(test_scene::SetupPlugin {})
	.add_system(bevy::window::close_on_esc)
	.add_startup_system(player::spawn_player)
	.run();
}
