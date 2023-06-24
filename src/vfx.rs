use bevy::prelude::*;

use crate::post_processing::effects::{flip, dither, feedback};
use crate::post_processing::setup::{EffectOutput, link_texture};
use crate::post_processing::{spawn_effect, self, link_effect};
use crate::post_processing::VFXChangeSystemSet;
use crate::resources::MainRenderTexture;

pub struct VFXPlugin {}

impl Plugin for VFXPlugin {
	fn build(&self, app: &mut App) {
		app
		//.add_plugin(post_processing::EffectPlugin::<test::Effect>::default())
		.add_plugin(post_processing::EffectPlugin::<flip::Effect>::default())
		.add_plugin(post_processing::EffectPlugin::<dither::Effect>::default())
		//.add_plugin(post_processing::EffectPlugin::<jpeg::Encode>::default())
		//.add_plugin(post_processing::EffectPlugin::<jpeg::Decode>::default())
		.add_plugin(post_processing::EffectPlugin::<feedback::Effect>::default())
		.add_startup_system(vfx_setup)
		.add_system(update_effects.in_set(VFXChangeSystemSet));
	}
}

fn vfx_setup(world: &mut World) {
	let window = world.query::<&Window>().single(world).clone();

	let render_target = world.get_resource::<MainRenderTexture>().expect("There needs to be a MainRenderTexture").clone();

	let feedback = spawn_effect::<feedback::Effect>(world, 30, EffectOutput::Texture {width: 1280, height: 720});
	link_texture::<feedback::Effect>(world, render_target.texture, feedback, 0);

	let dither = spawn_effect::<dither::Effect>(world, 29, EffectOutput::Texture {width: 640, height: 360});

	link_effect::<dither::Effect>(world, feedback, dither, 0);
	link_effect::<feedback::Effect>(world, dither, feedback, 1);

	let flip = spawn_effect::<flip::Effect>(world, 31, EffectOutput::Window {output_window: window});
	link_effect::<flip::Effect>(world, feedback, flip, 0);
}

fn update_effects(
	time: Res<Time>,
	mut objects: Query<&mut feedback::Effect>
) {
	for mut e in objects.iter_mut() {
		e.time = time.elapsed_seconds();
	}
}
