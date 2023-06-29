use bevy::prelude::*;
use crate::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_plugin(bevy_kira_audio::AudioPlugin)
		.init_resource::<CursorCabinPosition>()
		.init_resource::<ProgressBar>()
		.add_event::<ButtonPressEvent>()
		.add_event::<ThoughtCutsceneEndEvent>()
		.add_startup_systems((
			spawn_player_ship_audio,
		))
		.add_systems((
			update_player_audio,
		))
		;
	}
}

#[derive(Component)]
struct ShipAudio(Handle<AudioInstance>);

fn spawn_player_ship_audio(mut commands: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
	let handle = audio
	.play(asset_server.load("audio/effects/hyper_space_sounds.mp3"))
	.looped()
	.with_volume(0.0)
	.handle();
	commands.spawn(ShipAudio(handle));
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
