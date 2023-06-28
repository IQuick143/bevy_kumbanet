use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};
#[cfg(debug_assertions)]
use bevy_editor_pls::EditorPlugin;

use crate::{prelude::*, thoughts::data::ThoughtLibrary};
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_plugin(EditorPlugin::default())
			.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
			.add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
			.add_systems((
				clear_on_refresh,
				give_random_thought,
				debug_buttons,
				adjust_progress,
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

fn give_random_thought(
	keyboard: Res<Input<KeyCode>>,
	player: Query<Entity, With<Player>>,
	thoughts: Res<ThoughtLibrary>,
	mut event: EventWriter<ThoughtCollectedEvent>
) {
	if keyboard.just_pressed(KeyCode::G) {
		let mut rng = rand::thread_rng();
		event.send(ThoughtCollectedEvent {
		    player: player.single(),
		    thought: thoughts.get_thought_by_index(rand::Rng::gen::<usize>(&mut rng) % thoughts.n_thoughts()),
		});
	}
}

pub fn debug_buttons(
	mut click_events: EventReader<ButtonPressEvent>
) {
	for e in click_events.iter() {
		println!("Button {:?} pressed", e.button_type);
	}
}

fn adjust_progress(
	mut progress_bar: ResMut<ProgressBar>,
	keyboard: Res<Input<KeyCode>>,
) {
	if keyboard.just_pressed(KeyCode::O) {
		progress_bar.good_progress -= 0.1;
	}
	if keyboard.just_pressed(KeyCode::P) {
		progress_bar.good_progress += 0.1;
	}
	if keyboard.just_pressed(KeyCode::K) {
		progress_bar.bad_progress -= 0.1;
	}
	if keyboard.just_pressed(KeyCode::L) {
		progress_bar.bad_progress += 0.1;
	}
}