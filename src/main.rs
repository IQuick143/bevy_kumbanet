use bevy::prelude::*;

mod post_processing;
mod test_scene;

fn main() {
	let default_plugins = DefaultPlugins;
	
	#[cfg(debug_assertions)]
	let default_plugins = default_plugins.set(AssetPlugin {
		watch_for_changes: true,
		..Default::default()
	});

	App::new()
	.add_plugins(default_plugins)
	.add_plugin(test_scene::SetupPlugin {})
	.add_system(bevy::window::close_on_esc)
	.run();
}
