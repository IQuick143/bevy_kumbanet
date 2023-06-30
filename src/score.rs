use bevy::prelude::*;
use crate::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
	fn build(&self, app: &mut App) {
		app
		.insert_resource(ScoreCounter {
			score: 0,
			timer: Timer::from_seconds(13.0, TimerMode::Repeating),
		})
		.add_systems((
			update_counter_timer,
			add_event_scores,
		).distributive_run_if(in_state(crate::GameState::Game))
		)
		.add_system(tick_counter.in_schedule(CoreSchedule::FixedUpdate).run_if(in_state(crate::GameState::Game)))
        // configure our fixed timestep schedule to run twice a second
        .insert_resource(FixedTime::new_from_secs(1.0/60.0))
		;
	}
}

fn tick_counter(
	mut counter: ResMut<ScoreCounter>,
) {
	counter.score += 1;
}

fn update_counter_timer(
	mut counter: ResMut<ScoreCounter>,
	time: Res<Time>
) {
	counter.timer.tick(time.delta());
}

fn add_event_scores(
	mut counter: ResMut<ScoreCounter>,
	mut collections: EventReader<ThoughtCollectedEvent>,
	mut mergers: EventReader<ThoughtCutsceneEndEvent>,
) {
	for _ in collections.iter() {
		counter.score += 10000;
	}
	for _ in mergers.iter() {
		counter.score += 100000;
	}
}
