use bevy::prelude::*;

mod prelude {
	pub use crate::resources::*;
	pub use crate::components::*;
	pub use crate::events::*;
}

mod post_processing;
mod components;
mod test_scene;
mod resources;
mod animation;
mod thoughts;
mod physics;
mod player;
mod events;
mod score;
mod cabin;
mod audio;
mod boot;
mod vfx;

#[cfg(debug_assertions)]
mod debug;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
	#[default]
	Boot,
	Game,
}

fn main() {
	let default_plugins = DefaultPlugins;

	let default_plugins = default_plugins.set(WindowPlugin {
		primary_window: Some(Window {
			title: "KUMBANET client v0.3".to_string(),
			..Default::default()
		}), ..Default::default()
	});

	#[cfg(debug_assertions)]
	let default_plugins = default_plugins.set(AssetPlugin {
		watch_for_changes: true,
		..Default::default()
	});

	let mut app = App::new();

	app
		.add_plugins(default_plugins)
		.add_state::<GameState>()
		.init_resource::<resources::MainRenderTexture>()
		.add_plugin(boot::BootPlugin {})
		.add_plugin(cabin::CabinPlugin {})
		.add_plugin(audio::AudioPlugin {})
		.add_plugin(score::ScorePlugin {})
		.add_plugin(animation::AnimationPlugin {})
		.add_plugin(physics::PhysicsPlugin {})
		.add_plugin(player::PlayerBehaviourPlugin {})
		.add_plugin(thoughts::ThoughtsPlugin {})
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
