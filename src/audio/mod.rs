use std::time::Duration;

use bevy::prelude::*;
use crate::{prelude::*, GameState};
use bevy_kira_audio::prelude::*;

use self::slang::SlangTriggerEvent;

mod slang;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_plugin(bevy_kira_audio::AudioPlugin)
		.init_resource::<CursorCabinPosition>()
		.init_resource::<ProgressBar>()
		.add_event::<ButtonPressEvent>()
		.add_event::<SlangTriggerEvent>()
		.add_event::<ThoughtCutsceneEndEvent>()
		.add_systems((
			spawn_player_ship_audio,
			spawn_music,
		).in_schedule(OnEnter(GameState::Game)))
		.add_systems((
			update_player_audio,
			update_music_volume,
			slang::clean_up_slang_audio,
			slang::play_slang_audio,
			slang::try_trigger_slang,
		).distributive_run_if(in_state(GameState::Game)))
		;
	}
}

#[derive(Component)]
struct ShipAudio(Handle<AudioInstance>);

#[derive(Component)]
struct MusicPlayer(Handle<AudioInstance>);

fn spawn_player_ship_audio(mut commands: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
	let handle = audio
	.play(asset_server.load("audio/effects/hyper_space_sounds.mp3"))
	.looped()
	.with_volume(0.0)
	.handle();
	commands.spawn(ShipAudio(handle));
}


fn spawn_music(mut commands: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
	let handle = audio
	.play(asset_server.load("audio/effects/hyper_space_sounds.mp3"))
	.looped()
	.handle();
	commands.spawn(MusicPlayer(handle));
}

fn update_player_audio(
	player: Query<&Velocity, With<Player>>,
	ship_audio: Query<&ShipAudio>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
	if let Ok(ShipAudio(audio_instance)) = ship_audio.get_single() {
		if let Some(audio_instance) = audio_instances.get_mut(audio_instance) {
			let volume = if let Ok(velocity) = player.get_single() {
				let normalized_velocity = velocity.length() / 100.0;
				if normalized_velocity < 1.0 {
					0.0
				} else {
					normalized_velocity.ln()
				}
			} else {0.0};
			audio_instance.set_volume(volume as f64, AudioTween::default());
		}
	}
}

fn update_music_volume(
	priority_entities: Query<&PrioritySpeaker>,
	music: Query<&MusicPlayer>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
	let mut should_be_quiet = false;
	for _ in priority_entities.iter() {
		should_be_quiet = true;
		break;
	}
	let volume = if should_be_quiet {
		0.05
	} else {
		1.0
	};
	if let Ok(MusicPlayer(audio_instance)) = music.get_single() {
		if let Some(audio_instance) = audio_instances.get_mut(audio_instance) {
			audio_instance.set_volume(volume as f64, AudioTween::linear(Duration::from_millis(750)));
		}
	}
}
