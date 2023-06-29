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
	pub a: Box<dyn AnimationPath>,
	pub b: Box<dyn AnimationPath>,
}
impl AnimationPath for Sum {
	fn get_point(&self, t: f32) -> bevy::prelude::Vec3 {
		self.a.get_point(t) + self.b.get_point(t)
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Curtain {
	// How much the curtain should move from being open to being closed
	pub movement: Vec3,
	// time till closing or opening
	pub half_time: f32,
}
impl AnimationPath for Curtain {
	fn get_point(&self, t: f32) -> bevy::prelude::Vec3 {
		if t <= 0.0 {
			return self.movement;
		} else if t >= 2.0 * self.half_time {
			return self.movement;
		} else {
			let x = t / self.half_time - 1.0;
			return self.movement * x * x;
		}
	}
}
