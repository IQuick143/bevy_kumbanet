use bevy::prelude::*;

mod post_processing;
mod components;
mod test_scene;
mod resources;
mod physics;
mod player;
mod vfx;

#[cfg(debug_assertions)]
mod debug;

fn main() {
	let default_plugins = DefaultPlugins;
	
	#[cfg(debug_assertions)]
	let default_plugins = default_plugins.set(AssetPlugin {
		watch_for_changes: true,
		..Default::default()
	});

	let mut app = App::new();

	app
		.add_plugins(default_plugins)
		.init_resource::<resources::MainRenderTexture>()
		.add_plugin(physics::PhysicsPlugin {})
		.add_plugin(player::PlayerBehaviourPlugin {})
		.add_plugin(test_scene::SetupPlugin {})
		.add_plugin(vfx::VFXPlugin {})
		.add_startup_system(player::spawn_player_and_cameras)
	;
	
	{
		#[cfg(debug_assertions)]
		app.add_plugin(debug::DebugPlugin);
	}
	
	app.run();
}
