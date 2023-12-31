use bevy::prelude::*;

use crate::{prelude::*, thoughts::data::ThoughtLibrary, GameState, physics::PhysicsSystemSet};
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
			//.add_plugin(EditorPlugin::default())
			.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
			.add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
			.add_systems((
				give_random_thought,
				debug_buttons,
				debug_choreography_stops,
				adjust_progress,
			).distributive_run_if(in_state(GameState::Game)))
			.add_system(set_game_state)
			.add_system(player_interaction.after(PhysicsSystemSet).run_if(in_state(GameState::Game)))
		;
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

pub fn debug_choreography_stops(
	mut events: EventReader<ChoreographyStopEvent>
) {
	for e in events.iter() {
		println!("Choreography stopped, director_id:{:?}", e.director);
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

pub fn set_game_state(
	keyboard: Res<Input<KeyCode>>,
	state: Res<State<GameState>>,
	mut next_state: ResMut<NextState<GameState>>,
) {
	if keyboard.just_pressed(KeyCode::Z) {
		if state.0 != GameState::Boot {
			next_state.set(GameState::Boot);
			println!("Game State Boot");
		}
	}
	if keyboard.just_pressed(KeyCode::X) {
		if state.0 != GameState::Game {
			next_state.set(GameState::Game);
			println!("Game State Game");
		}
	}
}

fn player_interaction(
	mut events: EventReader<PlayerInteractionEvent>
) {
	for _e in events.iter() {
		println!("Player is hitting smth!!!");
	}
}
