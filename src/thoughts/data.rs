use bevy::prelude::Resource;

use super::Thought;
use super::Emotion::*;
use super::ThoughtWord::*;

#[derive(Resource, Clone)]
pub struct ThoughtLibrary {
	data: Vec<Thought>
}

const NO_SOUND: Option::<String> = None;

impl ThoughtLibrary {
	pub fn new() -> Self {
		ThoughtLibrary {data: vec![
			Thought::new("thoughts/images/debug/1.png",  NO_SOUND, 1, Positive, Noun("1".into())),
			Thought::new("thoughts/images/debug/-1.png", NO_SOUND, 1, Negative, Noun("-1".into())),
			Thought::new("thoughts/images/debug/2.png",  NO_SOUND, 2, Positive, Noun("2".into())),
			Thought::new("thoughts/images/debug/-2.png", NO_SOUND, 2, Negative, Noun("-2".into())),
		]}
	}

	pub fn n_thoughts(&self) -> usize {
		self.data.len()
	}

	pub fn get_thought_by_index(&self, index: usize) -> Thought {
		self.data[index].clone()
	}
}

impl Default for ThoughtLibrary {
    fn default() -> Self {Self::new()}
}
