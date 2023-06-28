use bevy::prelude::*;
use super::AnimationPath;

pub struct Stationary;
impl AnimationPath for Stationary {
	fn get_point(&self, _t: f32) -> bevy::prelude::Vec3 {
		return Vec3::ZERO;
	}
}

pub struct Sine {
	pub direction: Vec3,
	pub angular_frequency: f32,
}
impl AnimationPath for Sine {
	fn get_point(&self, t: f32) -> bevy::prelude::Vec3 {
		return f32::sin(self.angular_frequency * t) * self.direction;
	}
}
