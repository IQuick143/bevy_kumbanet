use std::time::Duration;

use bevy::prelude::*;
use rand::seq::SliceRandom;
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
			update_track,
			play_button_sounds,
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
struct MusicPlayer {
	track: usize,
	handle: Handle<AudioInstance>
}

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
	.play(asset_server.load(SONGS[0].0))
	.looped()
	.handle();
	commands.spawn(MusicPlayer {track: 0, handle});
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
	let should_be_quiet = !priority_entities.is_empty();
	let volume = if should_be_quiet {
		0.05
	} else {
		1.0
	};
	if let Ok(MusicPlayer {track, handle}) = music.get_single() {
		if let Some(audio_instance) = audio_instances.get_mut(handle) {
			if *track == POST_GAME_TRACKS {
				audio_instance.set_volume(1.0_f64, AudioTween::linear(Duration::from_millis(750)));
			} else {
				audio_instance.set_volume(volume as f64, AudioTween::linear(Duration::from_millis(750)));
			}
		}
	}
}

const SONGS: [(&str, u32); 4] = [
	("audio/music/cereal/corpo_blorpo.mp3", 0),
	("audio/music/cereal/top_10.mp3", 100000),
	("audio/music/cereal/psych_thing.mp3", 300000),
	("audio/music/iquick143/HealthAndPaintingInspection_2.mp3", 700000),
];

const JUKEBOX: [(&str, f32); 5] = [
	("audio/music/cereal/corpo_blorpo.mp3", 0.1),
	("audio/music/cereal/top_10.mp3", 0.3),
	("audio/music/cereal/psych_thing.mp3", 0.5),
	("audio/music/iquick143/HealthAndPaintingInspection_2.mp3", 0.2),
	("audio/music/iquick143/TheThingsWeNeverGotAroundTo.mp3", 0.1),
];

const ENDING_SONG: &str = "audio/music/iquick143/TheThingsWeNeverGotAroundTo.mp3";

const POST_GAME_TRACKS: usize = 255;

fn update_track(
	score: Option<Res<ScoreCounter>>,
	mut music: Query<&mut MusicPlayer>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>
) {
	if let Some(score) = score {
		let value = score.score;

		// Post-game jukebox
		if value > 1000000 {
			for mut player in music.iter_mut() {
				if player.track != POST_GAME_TRACKS {
					if let Some(instance) = audio_instances.get_mut(&player.handle) {
						instance.stop(AudioTween::linear(Duration::from_secs(3)));
					}
					let handle = audio
					.play(asset_server.load(ENDING_SONG))
					.fade_in(AudioTween::linear(Duration::from_secs(1)))
					.handle();
					player.handle = handle;
					player.track = POST_GAME_TRACKS;
					continue;
				}
				if let Some(instance) = audio_instances.get_mut(&player.handle) {
					if instance.state() == PlaybackState::Stopped {
						// Roll a new track
						let mut rng = rand::thread_rng();
						if let Ok(&(track,_)) = JUKEBOX.choose_weighted(&mut rng, |item| item.1) {
							let handle = audio
							.play(asset_server.load(track))
							.fade_in(AudioTween::linear(Duration::from_secs(1)))
							.handle();
							player.handle = handle;
						}
					}
				}
			}
			return;
		}

		// In-game progression based
		for mut player in music.iter_mut() {
			for (i, &(song_name, needed_score)) in SONGS.iter().enumerate().rev() {
				if needed_score < value && player.track < i {
					if let Some(instance) = audio_instances.get_mut(&player.handle) {
						instance.stop(AudioTween::linear(Duration::from_secs(5)));
					}
					let handle = audio
					.play(asset_server.load(song_name))
					.fade_in(AudioTween::linear(Duration::from_secs(3)))
					.looped()
					.handle();
					player.handle = handle;
					player.track = i;
				}
			}
		}
	}
}

fn play_button_sounds(
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	mut button_events: EventReader<ButtonPressEvent>
) {
	for e in button_events.iter() {
		if e.button_type == ButtonType::MergeThoughts {
			audio.play(asset_server.load("audio/effects/button.ogg")).with_volume(0.5);
		}
	}
}
