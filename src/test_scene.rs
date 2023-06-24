use bevy::prelude::*;

use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::components::AngularVelocity;

pub struct SetupPlugin {}

impl Plugin for SetupPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_startup_system(scene_setup);
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
