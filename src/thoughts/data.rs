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
//			Thought::new("thoughts/images/debug/1.png",  NO_SOUND, 1, Positive, Noun("1".into())),
//			Thought::new("thoughts/images/debug/-1.png", NO_SOUND, 1, Negative, Noun("-1".into())),
//			Thought::new("thoughts/images/debug/2.png",  NO_SOUND, 2, Positive, Noun("2".into())),
//			Thought::new("thoughts/images/debug/-2.png", NO_SOUND, 2, Negative, Noun("-2".into())),
			Thought::new("thoughts/images/openclipart/abstract_343040.png",		NO_SOUND, 1, Positive, Noun("The House of the Bird".into())),
			Thought::new("thoughts/images/openclipart/death_horse_313177.png",	NO_SOUND, 2, Negative, Noun("Incoming death".into())),
			Thought::new("thoughts/images/openclipart/gamer_grind_214410.png",	NO_SOUND, 1, Positive, Noun("gamer".into())),
			Thought::new("thoughts/images/openclipart/orange_juice_174090.png",	NO_SOUND, 1, Positive, Noun("orange juice".into())),
			Thought::new("thoughts/images/openclipart/grape_juice_343001.png",	NO_SOUND, 1, Positive, Noun("grape juice".into())),
			Thought::new("thoughts/images/openclipart/red_astronaut_279322.png",NO_SOUND, 1, Positive, Noun("astronaut".into())),
			Thought::new("thoughts/images/openclipart/angry_man_278871.png",	NO_SOUND, 1, Negative, Verb("angers".into())),
			Thought::new("thoughts/images/openclipart/forest_fire_327561.png",	NO_SOUND, 2, Negative, Noun("forest fire".into())),
			Thought::new("thoughts/images/openclipart/lighter_343041.png",		NO_SOUND, 1, Positive, Noun("lighter".into())),
			Thought::new("thoughts/images/openclipart/pain_319678.png",			NO_SOUND, 2, Negative, Noun("pain".into())),
			Thought::new("thoughts/images/openclipart/brain_306149.png",		NO_SOUND, 1, Positive, Noun("brain".into())),
			Thought::new("thoughts/images/openclipart/prismatic_mind_327002.png",NO_SOUND,3, Positive, Noun("ascended mind".into())),
			Thought::new("thoughts/images/openclipart/screw_191883.png",		NO_SOUND, 1, Negative, Noun("screw".into())),
			Thought::new("thoughts/images/wikimedia/abstract_deities.jpg",		NO_SOUND, 1, Positive, Noun("deity".into())),
			Thought::new("thoughts/images/wikimedia/bose_einstein.jpg",			NO_SOUND, 1, Positive, Noun("technology".into())),
			Thought::new("thoughts/images/wikimedia/cursed_stick_figures.jpg",	NO_SOUND, 2, Negative, Noun("Them".into())),
			Thought::new("thoughts/images/flickr/aligator.jpg",					NO_SOUND, 2, Negative, Noun("aligator".into())),
			Thought::new("thoughts/images/flickr/chair.jpg",					NO_SOUND, 1, Negative, Noun("chair".into())),
			Thought::new("thoughts/images/flickr/clouds.jpg",					NO_SOUND, 1, Positive, Noun("clouds".into())),
			Thought::new("thoughts/images/flickr/decay.jpg",					NO_SOUND, 2, Negative, Verb("decay".into())),
			Thought::new("thoughts/images/flickr/despair.jpg",					NO_SOUND, 2, Negative, Verb("despair".into())),
			Thought::new("thoughts/images/flickr/fog_man.jpg",					NO_SOUND, 1, Negative, Noun("Him".into())),
			Thought::new("thoughts/images/flickr/lock.jpg",						NO_SOUND, 1, Positive, Noun("lock".into())),
			Thought::new("thoughts/images/flickr/love.jpg",						NO_SOUND, 2, Positive, Verb("love".into())),
			Thought::new("thoughts/images/flickr/nature.jpg",					NO_SOUND, 2, Positive, Noun("nature".into())),
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
