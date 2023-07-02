use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::prelude::*;
use bevy_kira_audio::prelude::*;

pub(super) struct SlangTriggerEvent;

#[derive(Component)]
pub(super) struct SlangAudio(Handle<AudioInstance>);

pub(super) fn try_trigger_slang(
	slang_query: Query<Entity, With<SlangAudio>>,
	mut thought_collected: EventReader<ThoughtCollectedEvent>,
	mut thoughts_merged: EventReader<ThoughtCutsceneEndEvent>,
	mut trigger: EventWriter<SlangTriggerEvent>,
) {
	if !slang_query.is_empty() {
		return;
	}

	let mut do_it = false;
	let mut rng = thread_rng();

	for _ in thought_collected.iter() {
		if rng.gen_bool(0.075) {
			do_it = true;
		}
	}

	for _ in thoughts_merged.iter() {
		if rng.gen_bool(0.25) {
			do_it = true;
		}
	}

	// Idk he can talk by himself too sometimes
	if rng.gen_bool(0.0025 * 0.016) {
		do_it = true;
	}

	if do_it {
		trigger.send(SlangTriggerEvent);
	}
}

pub(super) fn play_slang_audio(
	mut commands: Commands,
	mut events: EventReader<SlangTriggerEvent>,
	audio: Res<Audio>,
	asset_server: Res<AssetServer>
) {
	if events.is_empty() {
		return;
	}
	events.clear();

	let mut rng = thread_rng();

	let tape_choice = rng.gen::<u32>() % 18;

	let handle = audio
	.play(asset_server.load(format!("audio/slang/slang_-{:0>2}.ogg", tape_choice+1)))
	.with_volume(2.0)
	.handle();
	commands.spawn(SlangAudio(handle)).insert(PrioritySpeaker);
}

pub(super) fn clean_up_slang_audio(
	mut commands: Commands,
	slang_query: Query<(Entity, &SlangAudio)>,
	audio_instances: Res<Assets<AudioInstance>>,
) {
	for (e, SlangAudio(audio_instance)) in slang_query.iter() {
		if let Some(audio_instance) = audio_instances.get(audio_instance) {
			if audio_instance.state() == PlaybackState::Stopped {
				commands.entity(e).despawn();
			}
		}
	}
}
