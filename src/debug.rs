use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy_editor_pls::EditorPlugin;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_plugin(EditorPlugin::default())
			.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
			.add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
		;
	}
}