use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};
#[cfg(debug_assertions)]
use bevy_editor_pls::EditorPlugin;

use crate::prelude::ClearCamera;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_plugin(EditorPlugin::default())
			.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
			.add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
			.add_systems((
				clear_on_refresh,
			))
		;
	}
}

fn clear_on_refresh(
	keyboard: Res<Input<KeyCode>>,
	mut clear_camera_query: Query<(&mut Camera3d, &mut ClearCamera)>,
) {
	if keyboard.just_pressed(KeyCode::C) {
		let (mut camera, mut clear_toggle) = clear_camera_query.single_mut();
		if !clear_toggle.0 {
			camera.clear_color = default();
		} else {
			camera.clear_color = ClearColorConfig::None;
		}
		clear_toggle.0 = !clear_toggle.0;
	}
}