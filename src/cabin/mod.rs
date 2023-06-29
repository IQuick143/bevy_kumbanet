use bevy::{prelude::*, render::{view::RenderLayers, camera::{RenderTarget, ScalingMode}}, core_pipeline::clear_color::ClearColorConfig, math::Vec3Swizzles};

use crate::prelude::*;

mod ui;

pub const CABIN_WIDTH: f32 = 16.0;
pub const CABIN_HEIGHT: f32 = 9.0;

pub struct CabinPlugin;

impl Plugin for CabinPlugin {
	fn build(&self, app: &mut App) {
		app
		.init_resource::<CursorCabinPosition>()
		.init_resource::<ProgressBar>()
		.add_event::<ButtonPressEvent>()
		.add_event::<ThoughtCutsceneEndEvent>()
		.add_startup_systems((
			spawn_cabin_camera,
			ui::spawn_ui,
			ui::spawn_bar,
			ui::spawn_score_counter,
		))
		.add_systems((
			update_cursor_position,
			ui::track_cursor,
			spawn_collected_thoughts,
			move_cabin_thoughts,
			ui::check_buttons,
		).chain())
		.add_systems((
			start_thought_animation.before(crate::animation::AnimationSystemSet),
			check_cutscene_end.after(crate::animation::AnimationSystemSet),
			ui::update_progress_bar,
			ui::update_score_text,
		))
		;
	}
}

fn spawn_cabin_camera(
	mut commands: Commands,
	render_target: Res<MainRenderTexture>,
) {
	commands.spawn((
		Camera2dBundle {
			camera: Camera {
				//viewport: Some(Viewport {physical_position: UVec2::new(0, 0 /*size.x/2, size.y/2*/), physical_size: size, ..Default::default()}),
				target: RenderTarget::Image(render_target.texture.clone()),
				order: 10, is_active: true, ..Default::default()
			},
			projection: OrthographicProjection {scaling_mode: ScalingMode::Fixed{width: CABIN_WIDTH, height: CABIN_HEIGHT}, ..Default::default()},
			camera_2d: Camera2d {clear_color: ClearColorConfig::None},
			..Default::default()
		},
		CabinCamera,
		RenderLayers::layer(1),
		Name::new("Cabin Camera"),
	));
}

fn spawn_collected_thoughts(
	mut commands: Commands,
	assets: Res<AssetServer>,
	mut collected_thoughts: EventReader<ThoughtCollectedEvent>,
	mut progress_bar: ResMut<ProgressBar>,
) {
	for ThoughtCollectedEvent {player: _, thought} in collected_thoughts.iter() {
		match thought.emotion {
			crate::thoughts::Emotion::Positive => progress_bar.good_progress += 0.1,
			crate::thoughts::Emotion::Negative => progress_bar.bad_progress += 0.1,
		}
		spawn_cabin_thought(&mut commands, &assets, thought.clone());
	}
}

fn spawn_cabin_thought(
	commands: &mut Commands,
	assets: &AssetServer,
	thought: Thought,
) {
	commands.spawn((
		SpriteBundle {
			sprite: Sprite {custom_size: Some(Vec2::new(1.0, 1.0)), ..Default::default()},
			texture: thought.load_image(assets),
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
			..Default::default()
		},
		RenderLayers::layer(1),
		Name::new(format!("CabinThought {:?}", thought.word)),
		Velocity(Vec3::new(1.0,1.0,0.0)),
		VelocityDrag(0.05),
		CabinThought(thought),
	));
}

fn move_cabin_thoughts(
	mut cabin_thought_query: Query<(&mut Transform, &mut Velocity), With<CabinThought>>,
	angular_velocity_query: Query<&AngularVelocity, With<Player>>,
	mouse_pos: Res<CursorCabinPosition>,
	inputs: Res<Input<MouseButton>>,
	time: Res<Time>
) {
	let angular_velocity = if let Ok(angular) = angular_velocity_query.get_single() {
		angular.length()
	} else {0.0};

	for (mut transform, mut velocity) in cabin_thought_query.iter_mut() {
		let mut colliding = false;
		if transform.translation.x < -CABIN_WIDTH / 2.0 && velocity.x < 0.0 {
			transform.translation = Vec3::new(-CABIN_WIDTH / 2.0, transform.translation.y, transform.translation.z);
			velocity.0 = Vec3::new(-velocity.x, velocity.y, velocity.z);
			colliding = true;
		} else if transform.translation.x > CABIN_WIDTH / 2.0 && velocity.x > 0.0 {
			transform.translation = Vec3::new(CABIN_WIDTH / 2.0, transform.translation.y, transform.translation.z);
			velocity.0 = Vec3::new(-velocity.x, velocity.y, velocity.z);
			colliding = true;
		}
		if transform.translation.y < -CABIN_HEIGHT / 2.0 && velocity.y < 0.0 {
			transform.translation = Vec3::new(transform.translation.x, -CABIN_HEIGHT / 2.0, transform.translation.z);
			velocity.0 = Vec3::new(velocity.x, -velocity.y, velocity.z);
			colliding = true;
		} else if transform.translation.y > CABIN_HEIGHT / 2.0 && velocity.y > 0.0 {
			transform.translation = Vec3::new(transform.translation.x, CABIN_HEIGHT / 2.0, transform.translation.z);
			velocity.0 = Vec3::new(velocity.x, -velocity.y, velocity.z);
			colliding = true;
		}
		if !colliding {
			let r = transform.translation.xy().extend(0.0);
			velocity.0 += time.delta_seconds() * 0.1 * angular_velocity * angular_velocity * r;
		}
	}

	if inputs.any_pressed([MouseButton::Left, MouseButton::Middle, MouseButton::Right]) {
		for (transform, mut velocity) in cabin_thought_query.iter_mut() {
			let r = (transform.translation.xy() - mouse_pos.world_position).extend(0.0);
			let r_square = r.length_squared();
			if r_square > 0.01 {
				velocity.0 += -(time.delta_seconds() * 3.0 / r_square) * r.normalize()
			}
		}
	}
}

fn start_thought_animation(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut start_event: EventReader<ButtonPressEvent>,
	other_director: Query<Entity, With<CabinCutsceneDirector>>,
	thought_query: Query<Entity, (With<CabinThought>, Without<crate::animation::AnimatedObject>)>,
) {
	use crate::animation::*;

	// If a director is present, break
	for _ in other_director.iter() {
		return;
	}

	let mut start = false;
	for event in start_event.iter() {
		if event.button_type == ButtonType::MergeThoughts {
			start = true;
		}
	}
	if !start {
		return;
	}

	let choreo = Choreography {
		// Three thoughts needed
		n_actors: 5, // NOTE: 0th and 1st are the left/right curtains
		// Centered on center screen
		initial_position: Vec3::new(0.0, 0.0, 0.0),
		data: vec![// NOTE: 0th and 1st are the curtains
			//Make the first actor stay in the center, second one orbit and third orbit differently
			(0.0, ChoreographyEvent::SetAnimation(2, Box::new(animations::Stationary))),
			(0.0, ChoreographyEvent::SetAnimation(3, Box::new(animations::Ellipse::circle(2.0, 0.5)))),
			(0.0, ChoreographyEvent::SetAnimation(4, Box::new(animations::Ellipse::circle(3.0, 0.25)))),
			// Curtains
			(0.0, ChoreographyEvent::SetActorsOffset(0, Vec3::new(-CABIN_WIDTH / 4.0,0.0,0.0))),
			(0.0, ChoreographyEvent::SetActorsOffset(1, Vec3::new( CABIN_WIDTH / 4.0,0.0,0.0))),
			(0.0, ChoreographyEvent::SetAnimation(0, Box::new(animations::Curtain {half_time: 2.5, movement: -Vec3::X * CABIN_WIDTH * 0.5}))),
			(0.0, ChoreographyEvent::SetAnimation(1, Box::new(animations::Curtain {half_time: 2.5, movement:  Vec3::X * CABIN_WIDTH * 0.5}))),
			// Start first thought
			(0.0, ChoreographyEvent::ActivateActor(0)),
			(0.0, ChoreographyEvent::ActivateActor(1)),
			(0.0, ChoreographyEvent::ActivateActor(2)),
			// Start second thought
			(5.0, ChoreographyEvent::ActivateActor(3)),
			// Start third thought
			(10.0, ChoreographyEvent::ActivateActor(4)),
			// Change the animation on the third thought to be smaller and faster
			(15.0, ChoreographyEvent::SetAnimation(4, Box::new(animations::Ellipse::circle(1.0, 2.0)))),
			// Change the animation on the second thought to be wilder
			(20.0, ChoreographyEvent::SetAnimation(3, Box::new(animations::Sum {
				a: Box::new(animations::Ellipse::circle(0.25, 4.0)),
				b: Box::new(animations::Ellipse {major_semiaxis:Vec3::new(4.0,0.0,0.0), minor_semiaxis:Vec3::new(2.0,2.0,0.0), frequency:1.0})
			}))),
			// Restart the curtains
			(27.5, ChoreographyEvent::SetActorsTime(0,0.0)),
			(27.5, ChoreographyEvent::SetActorsTime(1,0.0)),
			(30.0, ChoreographyEvent::EndChoreography)
		]
	};

	// Look for actors
	let mut actors = Vec::new();
	for actor in thought_query.iter() {
		if actors.len() >= choreo.n_actors - 2 {
			break;
		}
		actors.push(actor);
	}

	if actors.len() < choreo.n_actors - 2 {
		// Didnt find enough actors
		return;
	}
	
	let (left_curtain, right_curtain) = ui::spawn_curtains(&mut commands, asset_server);

	actors.insert(0, left_curtain);
	actors.insert(1, right_curtain);

	for actor_entity in actors.clone() {
		commands.entity(actor_entity).remove::<(CabinThought, Velocity)>();
	}

	let director = organize_play(&mut commands, choreo, actors);
	commands.entity(director).insert(CabinCutsceneDirector).insert(PrioritySpeaker);
}

fn update_cursor_position(
	window_query: Query<&Window>,
	mut cursor: ResMut<CursorCabinPosition>,
) {
	let window = window_query.get_single().expect("There should be a single window");
	if let Some(pos) = window.cursor_position() {
		cursor.uv_position = Vec2::new(
			pos.x / window.width(),
			pos.y / window.height(),
		);
		cursor.world_position = Vec2::new(
			(cursor.uv_position.x - 0.5) * CABIN_WIDTH,
			(cursor.uv_position.y - 0.5) *  CABIN_HEIGHT,
		);
	}
}

fn check_cutscene_end(
	mut in_event: EventReader<ChoreographyStopEvent>,
	mut event: EventWriter<ThoughtCutsceneEndEvent>,
	director: Query<Entity, With<CabinCutsceneDirector>>,
) {
	for e in in_event.iter() {
		if let Ok(_) = director.get(e.director) {
			event.send(ThoughtCutsceneEndEvent);
		}
	}
}
