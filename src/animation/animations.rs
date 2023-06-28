use bevy::prelude::*;
use super::AnimationPath;

#[derive(Clone, Copy, Debug)]
pub struct Stationary;
impl AnimationPath for Stationary {
	fn get_point(&self, _t: f32) -> bevy::prelude::Vec3 {
		return Vec3::ZERO;
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Ellipse {
	pub major_semiaxis: Vec3,
	pub minor_semiaxis: Vec3,
	pub frequency: f32,
}
impl Ellipse {
	// Makes an ellipse that is a circle in the X-Y plane
	pub fn circle(radius: f32, frequency: f32) -> Self {
		Ellipse {
			major_semiaxis: Vec3::new(radius, 0.0, 0.0),
			minor_semiaxis: Vec3::new(0.0, radius, 0.0),
			frequency,
		}
	}
}
impl AnimationPath for Ellipse {
	fn get_point(&self, t: f32) -> bevy::prelude::Vec3 {
		let p = 2.0 * std::f32::consts::PI * self.frequency * t;
		return f32::cos(p) * self.major_semiaxis + f32::sin(p) * self.minor_semiaxis;
	}
}

#[derive(Clone, Debug)]
pub struct Sum {
	a: Box<dyn AnimationPath>,
	b: Box<dyn AnimationPath>,
}
impl AnimationPath for Sum {
	fn get_point(&self, t: f32) -> bevy::prelude::Vec3 {
		self.a.get_point(t) + self.b.get_point(t)
	}
}
