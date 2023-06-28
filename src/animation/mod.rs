use std::fmt::Debug;
use bevy::prelude::*;

pub mod animations;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_systems((direct_play, animate_transforms).chain())
		;
	}
}


// A little cursed helper trait used to implement clone Box<dyn AnimationPath>
pub trait AnimationPathClone {
	fn clone_box(&self) -> Box<dyn AnimationPath>;
}

// We blanket implement it for everything that can be cloned
impl<T> AnimationPathClone for T
where
	T: 'static + AnimationPath + Clone,
{
	fn clone_box(&self) -> Box<dyn AnimationPath> {
		Box::new(self.clone())
	}
}

// We need to implement this to be able to clone the thing
impl Clone for Box<dyn AnimationPath> {
	fn clone(&self) -> Box<dyn AnimationPath> {
		self.clone_box()
	}
}

// Send + Sync needed for derive(Component) because thread safety
pub trait AnimationPath
where
	Self: Send + Sync + Debug + AnimationPathClone {
	fn get_point(&self, t: f32) -> Vec3;
}

#[derive(Component, Debug)]
pub struct AnimatedObject {
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

#[derive(Component, Clone, Debug)]
pub struct Choreography {
	pub n_actors: usize,
	pub initial_position: Vec3,
	pub data: Vec<(f32, ChoreographyEvent)>
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ChoreographyEvent {
	ActivateActor(usize),
	DeactivateActor(usize),
	SetAnimation(usize, Box<dyn AnimationPath>),
	SetActorsTime(usize, f32),
	SetActorsOffset(usize, Vec3),
}

#[derive(Component)]
pub struct Director {
	time: f32,
	actors: Vec<Entity>,
	choreography: Choreography
}

impl Director {
	// Gets the events in the given time_range
	pub fn get_events_in_time_range(&self, start: f32, end: f32) -> Vec<ChoreographyEvent> {
		self.choreography.data.iter().filter_map(|(time, action)| {
			if !(time < &start) && (time < &end) {
				Some(action.clone())
			} else {
				None
			}
		}).collect()
	}
}

pub fn organize_play(
	commands: &mut Commands,
	choreography: Choreography,
	actors_entities: Vec<Entity>,
) {
	if actors_entities.len() != choreography.n_actors {
		warn!("Incorrect amount of actors got: {}, expected: {}", actors_entities.len(), choreography.n_actors);
	}
	for entity in actors_entities.iter() {
		commands.entity(entity.clone()).insert(AnimatedObject {
			active: false,
			time: 0.0,
			offset: choreography.initial_position,
			animation: Box::new(animations::Stationary),
		});
	}
	commands.spawn(Director {
		time: 0.0,
		actors: actors_entities,
		choreography,
	});
}

fn direct_play(
	mut actors: Query<&mut AnimatedObject>,
	mut directors: Query<&mut Director>,
	time: Res<Time>
) {
	let dt = time.delta_seconds();
	for mut director in directors.iter_mut() {
		let t_0 = director.time;
		let t_1 = t_0 + dt;
		director.time = t_1;
		for event in director.get_events_in_time_range(t_0, t_1) {
			let actor_id = match event {
				ChoreographyEvent::ActivateActor(actor) => actor,
				ChoreographyEvent::DeactivateActor(actor) => actor,
				ChoreographyEvent::SetAnimation(actor, _) => actor,
				ChoreographyEvent::SetActorsTime(actor, _) => actor,
				ChoreographyEvent::SetActorsOffset(actor, _) => actor,
			};
			let actor_name = director.actors[actor_id];
			match actors.get_mut(actor_name) {
				Err(error) => warn!("Could not reach actor {}. Reason: {}", actor_id, error),
				Ok(mut actor) => {
					match event {
						ChoreographyEvent::ActivateActor(_) => actor.active = true,
						ChoreographyEvent::DeactivateActor(_) => actor.active = false,
						ChoreographyEvent::SetAnimation(_, animation) => actor.animation = animation,
						ChoreographyEvent::SetActorsTime(_, time) => actor.time = time,
						ChoreographyEvent::SetActorsOffset(_, offset) => actor.offset = offset,
					};
				},
			}
		}
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
