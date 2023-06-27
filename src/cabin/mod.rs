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
		.add_event::<ButtonPressEvent>()
		.add_startup_systems((
			spawn_cabin_camera,
			ui::spawn_ui,
		))
		.add_systems((
			update_cursor_position,
			ui::track_cursor,
			spawn_collected_thoughts,
			move_cabin_thoughts,
			ui::check_buttons,
		).chain())
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
) {
	for ThoughtCollectedEvent {player: _, thought} in collected_thoughts.iter() {
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
