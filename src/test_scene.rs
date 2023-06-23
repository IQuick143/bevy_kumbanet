use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureDescriptor, TextureUsages};
use bevy::render::texture::BevyDefault;
use bevy::render::view::RenderLayers;

use crate::components::AngularVelocity;
use crate::post_processing::effects::{test, flip, jpeg, dither, feedback};
use crate::post_processing::setup::{EffectOutput, link_texture};
use crate::post_processing::{spawn_effect, self, link_effect};
use crate::post_processing::VFXChangeSystemSet;

pub struct SetupPlugin {}

impl Plugin for SetupPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_plugin(post_processing::EffectPlugin::<test::Effect>::default())
		.add_plugin(post_processing::EffectPlugin::<flip::Effect>::default())
		.add_plugin(post_processing::EffectPlugin::<dither::Effect>::default())
		.add_plugin(post_processing::EffectPlugin::<jpeg::Encode>::default())
		.add_plugin(post_processing::EffectPlugin::<jpeg::Decode>::default())
		.add_plugin(post_processing::EffectPlugin::<feedback::Effect>::default())
		.add_startup_system(vfx_setup)
		.add_startup_system(scene_setup)
		.add_system(update_effects.in_set(VFXChangeSystemSet));
	}
}

fn scene_setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut images: ResMut<Assets<Image>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let star_texture = asset_server.load("images/stars.png");

	// this material renders the texture normally
	let star_material = materials.add(StandardMaterial {
		base_color_texture: Some(star_texture.clone()),
		alpha_mode: AlphaMode::Blend,
		unlit: true,
		..default()
	});

	let material = materials.add(StandardMaterial {
		base_color_texture: Some(images.add(generate_texture())),
		..default()
	});

	let cube = meshes.add(shape::Cube::default().into());

	commands.spawn(PbrBundle {
		mesh: cube,
		material: material,
		transform: Transform::from_xyz(0.0, 2.0, 0.0),
		..default()
	}).insert(AngularVelocity(Vec3::Y));

	commands.spawn(PointLightBundle {
		point_light: PointLight {
			intensity: 10000.0,
			range: 100.,
			shadows_enabled: true,
			..default()
		},
		transform: Transform::from_xyz(10.0, 16.0, 8.0),
		..default()
	});

	// ground plane
	commands.spawn(PbrBundle {
		mesh: meshes.add(
			shape::Plane {
				size: 50.,
				..default()
			}.into(),
		),
		material: star_material,
		..default()
	});
}

#[derive(Component, Hash, PartialEq, Eq, Default)]
pub struct MainCamera;

fn vfx_setup(world: &mut World) {
	let window = world.query::<&Window>().single(world).clone();

	let width = window.physical_width();
	let height = window.physical_height();

	let size = Extent3d {width, height, ..default()};

	// Image the main camera renders to
	let image_handle = {
		let mut image = Image {
			texture_descriptor: TextureDescriptor {
				label: None,
				size: size,
				dimension: TextureDimension::D2,
				format: TextureFormat::bevy_default(),
				mip_level_count: 1,
				sample_count: 1,
				usage: TextureUsages::TEXTURE_BINDING
					| TextureUsages::COPY_DST
					| TextureUsages::RENDER_ATTACHMENT,
				view_formats: &[],
			},
			..default()
		};

		// fill image.data with zeroes
		image.resize(size);
		world.resource_mut::<Assets<Image>>().add(image)
	};

	// The main camera
	world.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(0.0, 6.0, 12.0)
				.looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
			camera: Camera {
				target: RenderTarget::Image(image_handle.clone()),
				//target: RenderTarget::Window(bevy::window::WindowRef::Primary),
				..default()
			},
			..default()
		},
		RenderLayers::from_layers(&[0]),
		MainCamera
	));

	let feedback = spawn_effect::<feedback::Effect>(world, 30, EffectOutput::Texture {width: 1280, height: 720});
	link_texture::<feedback::Effect>(world, image_handle, feedback, 0);

	let dither = spawn_effect::<dither::Effect>(world, 29, EffectOutput::Texture {width: 640, height: 360});

	link_effect::<dither::Effect>(world, feedback, dither, 0);
	link_effect::<feedback::Effect>(world, dither, feedback, 1);

	let flip = spawn_effect::<flip::Effect>(world, 31, EffectOutput::Window {output_window: window});
	link_effect::<flip::Effect>(world, feedback, flip, 0);
}

fn update_effects(
	time: Res<Time>,
	mut objects: Query<&mut feedback::Effect>
) {
	for mut e in objects.iter_mut() {
		e.time = time.elapsed_seconds();
	}
}

// Taken from bevy_vfx_bag samples
fn generate_texture() -> Image {
	const TEXTURE_SIZE: usize = 8;

	let mut palette: [u8; 32] = [
		255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102,
		255, 198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
	];

	let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
	for y in 0..TEXTURE_SIZE {
		let offset = TEXTURE_SIZE * y * 4;
		texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
		palette.rotate_right(4);
	}

	Image::new_fill(
		Extent3d {
			width: TEXTURE_SIZE as u32,
			height: TEXTURE_SIZE as u32,
			depth_or_array_layers: 1,
		},
		TextureDimension::D2,
		&texture_data,
		TextureFormat::Rgba8UnormSrgb,
	)
}
