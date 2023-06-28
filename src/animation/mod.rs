use std::fmt::Debug;
use bevy::prelude::*;

use crate::prelude::ChoreographyStopEvent;

pub mod animations;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_event::<ChoreographyStopEvent>()
		.add_systems((direct_play, animate_transforms, clean_up).chain())
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

// Makes objects move along AnimationPaths
#[derive(Component, Debug)]
pub struct AnimatedObject {
	// Whether the animation is going
	pub active: bool,
	// Local time of the animation
	pub time: f32,
	// Coordinate origin for the object
	pub offset: Vec3,
	// The animation path object driving the animation
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
	// How many actors are in the choreography
	pub n_actors: usize,
	// Default animation offset
	pub initial_position: Vec3,
	// The choreography events with their time stamp in seconds (doesn't need to be in order)
	pub data: Vec<(f32, ChoreographyEvent)>
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ChoreographyEvent {
	// Starts the animation on the actor given by the id
	ActivateActor(usize),
	// Stops the animation on the actor given by the id
	DeactivateActor(usize),
	// Sets a new animation on the actor given by the id
	SetAnimation(usize, Box<dyn AnimationPath>),
	// Sets the time on the actor( given by the id)'s watch
	SetActorsTime(usize, f32),
	// Sets the animation offset (0 in their coordinates) for the actor given by the id
	SetActorsOffset(usize, Vec3),
	// Ends the choreography, but if you stage events after the end, I don't guarantee what will happen
	EndChoreography
}

#[derive(Component)]
pub struct Director {
	active: bool,
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

// Constructs a director with the given choreography and given actors
// And gives the actors the necesary AnimatedObject components
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
		active: true,
		time: 0.0,
		actors: actors_entities,
		choreography,
	});
}

// Updates animations based on directors choreographies
fn direct_play(
	mut actors: Query<&mut AnimatedObject>,
	mut directors: Query<(Entity, &mut Director)>,
	mut stop_events: EventWriter<ChoreographyStopEvent>,
	time: Res<Time>
) {
	let dt = time.delta_seconds();
	for (director_entity, mut director) in directors.iter_mut() {
		if !director.active {
			continue;
		}
		let t_0 = director.time;
		let t_1 = t_0 + dt;
		director.time = t_1;
		for event in director.get_events_in_time_range(t_0, t_1) {
			if let ChoreographyEvent::EndChoreography = event {
				stop_events.send(ChoreographyStopEvent {director: director_entity});
				director.active = false;
				
				for (actor_id, actor_name) in director.actors.iter().enumerate() {
					match actors.get_mut(*actor_name) {
						Err(error) => warn!("Could not reach actor {}. Reason: {}", actor_id, error),
						Ok(mut actor) => actor.active = false
					}
				}
				break;
			}
			let actor_id = match event {
				ChoreographyEvent::ActivateActor(actor) => actor,
				ChoreographyEvent::DeactivateActor(actor) => actor,
				ChoreographyEvent::SetAnimation(actor, _) => actor,
				ChoreographyEvent::SetActorsTime(actor, _) => actor,
				ChoreographyEvent::SetActorsOffset(actor, _) => actor,
				ChoreographyEvent::EndChoreography => unreachable!(),
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
						ChoreographyEvent::EndChoreography => unreachable!(),
					};
				},
			}
		}
	}
}

// Runs animations
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

fn clean_up(
	mut commands: Commands,
	mut stop_events: EventReader<ChoreographyStopEvent>,
//	actors: Query<Entity, With<AnimatedObject>>,
	directors: Query<&Director>,
) {
	for event in stop_events.iter() {
		if let Ok(director) = directors.get(event.director) {
			for entity in director.actors.iter() {
				commands.get_entity(*entity).map(|e| e.despawn_recursive());
			}
			commands.entity(event.director).despawn();
		}
	}
}
