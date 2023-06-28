use bevy::prelude::*;

pub mod animations;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system(animate_transforms)
		;
	}
}

// Send + Sync needed for derive(Component) because thread safety
trait AnimationPath where Self: Send + Sync {
	fn get_point(&self, t: f32) -> Vec3;
}

#[derive(Component)]
struct AnimatedObject {
	pub active: bool,
	pub time: f32,
	pub offset: Vec3,
	pub animation: Box<dyn AnimationPath>,
}

impl AnimatedObject {
	#[inline]
	fn get_current_point(&self) -> Vec3 {
		self.offset + self.animation.get_point(self.time)
	}
}

fn animate_transforms(
	mut objects: Query<(&mut Transform, &mut AnimatedObject)>,
	time: Res<Time>
) {
	for (mut transform, mut animation) in objects.iter_mut() {
		if animation.active {
			transform.translation = animation.get_current_point();
			animation.time += time.delta_seconds();
		}
	}
}
