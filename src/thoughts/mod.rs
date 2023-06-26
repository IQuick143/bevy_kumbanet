use bevy::prelude::*;

pub mod data;
mod systems;

use self::{systems::{spawn_thoughts, rotate_thoughts}, data::ThoughtLibrary};

pub struct ThoughtsPlugin;

impl Plugin for ThoughtsPlugin {
	fn build(&self, app: &mut App) {
		app
		.init_resource::<ThoughtLibrary>()
		.add_startup_systems((
			spawn_thoughts,
		))
		.add_systems((
			rotate_thoughts,
		));
	}
}

#[derive(Component, PartialEq, Eq, Clone, Debug)]
pub struct Thought {
	pub image: String,
	pub audio: Option<String>,
	pub intensity: u32,
	pub emotion: Emotion,
	pub word: ThoughtWord
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ThoughtWord {
	Noun(String),
	Verb(String),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Emotion {
	Positive,
	Negative
}

impl Thought {
	pub fn new(image: impl Into<String>, audio: Option<impl Into<String>>, intensity: u32, emotion: Emotion, word: ThoughtWord) -> Self {
		Thought {
			image: image.into(),
			audio: audio.map(Into::into),
			intensity,
			emotion,
			word
		}
	}

	pub fn load_image(&self, assets: &AssetServer) -> Handle<Image> {
		assets.load(self.image.clone())
	}

	pub fn create_material(&self, assets: &AssetServer) -> StandardMaterial {
		StandardMaterial {
			base_color_texture: Some(self.load_image(assets)),
			//alpha_mode: (),
			// depth_bias: () TODO: Consider using this for cursed effects
			..Default::default()
		}
	}

	/*pub fn load_sound(&self, assets: &AssetServer) -> Handle<> {
		audio: self.audio.clone().map(|audio_file| {assets.load(audio_file)}),
	}*/
}
