//! A custom post processing effect, using two cameras, with one reusing the render texture of the first one.
//! Here a chromatic aberration is applied to a 3d scene containing a rotating cube.
//! This example is useful to implement your own post-processing effect such as
//! edge detection, blur, pixelization, vignette... and countless others.

use bevy::{
	prelude::*,
	render::{view::RenderLayers, render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages},
	texture::BevyDefault, camera::RenderTarget}, sprite::MaterialMesh2dBundle,
};

use super::{PostProcessingEffect, PostProcessingEffectMaterial, EffectAssociatedCameraID};

#[derive(Debug, Default)]
pub struct VFXPlugin {}

impl Plugin for VFXPlugin {
	fn build(&self, app: &mut App) {
		app;
	}
}

#[derive(Clone)]
pub enum EffectOutput {
	Window {
		output_window: Window
	},
	Texture {
		width: u32,
		height: u32,
	}
}

pub fn spawn_effect<Effect: PostProcessingEffect>(
	world: &mut World,
	camera_layer: u8,
	output: EffectOutput,
) -> Entity {
	let size = match output.clone() {
		EffectOutput::Window {output_window} => Extent3d {
			width: output_window.resolution.physical_width(),
			height: output_window.resolution.physical_height(),
			..default()
		},
		EffectOutput::Texture {width, height} => Extent3d {
			width, height, ..default()
		}
	};

	let (image_handle, render_target) = match output {
	EffectOutput::Window {output_window: _} => (None, RenderTarget::Window(bevy::window::WindowRef::Primary)),
	EffectOutput::Texture {width: _, height: _} => {
		// This is the texture that will be rendered to.
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
		let image_handle = world.resource_mut::<Assets<Image>>().add(image);
		(Some(image_handle.clone()), RenderTarget::Image(image_handle))
	}};

	// This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
	let post_processing_pass_layer = RenderLayers::layer(camera_layer);

	let quad_handle = {
		let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
		meshes.add(Mesh::from(shape::Quad::new(Vec2::new(size.width as f32, size.height as f32))))
	};

	// This material has the texture that has been rendered.
	let material_handle = {
		let mut post_processing_materials = world.get_resource_mut::<Assets<Effect::MaterialType>>().expect("There should be a Material2DPlugin for the shader material");
		post_processing_materials.add(Effect::MaterialType::new())	
	};

	// The post-processing pass camera.
	let camera_id = world.spawn((
		Camera2dBundle {
			camera: Camera {
				// renders after the first main camera which has default value: 0.
				order: 1,
				target: render_target,
				..default()
			},
			..Camera2dBundle::default()
		},
		UiCameraConfig { show_ui: false },
		post_processing_pass_layer,
	)).id();

	// Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
	let mut effect_entity = world.spawn((
		Effect::from_handle(material_handle.clone()),
		MaterialMesh2dBundle {
			mesh: quad_handle.into(),
			material: material_handle,
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 1.5),
				..default()
			},
			..default()
		},
		post_processing_pass_layer,
		EffectAssociatedCameraID(camera_id)
	));

	if let Some(handle) = image_handle {
		effect_entity.insert(handle);
	}
	
	effect_entity.id()
}

pub fn link_effect<Effect: PostProcessingEffect>(
	world: &mut World,
	source_effect: Entity,
	destination_effect: Entity,
	destination_slot: usize,
) {
	let source_image = world.get::<Handle<Image>>(source_effect).expect("Effect should have an image (unless it's outputting to the window)").clone();
	let destination_material_handle = world.get::<Handle<Effect::MaterialType>>(destination_effect).expect("Effect entity has a corresponding Material component").clone();

	let mut materials = world.resource_mut::<Assets<Effect::MaterialType>>();
	materials.get_mut(&destination_material_handle).expect("Material should exist").set_slot(destination_slot, source_image).expect("Something went wrong idk");
}

pub fn link_texture<Effect: PostProcessingEffect>(
	world: &mut World,
	source_image: Handle<Image>,
	destination_effect: Entity,
	destination_slot: usize,
) {
	let destination_material_handle = world.get::<Handle<Effect::MaterialType>>(destination_effect).expect("Effect entity has a corresponding Material component").clone();

	let mut materials = world.resource_mut::<Assets<Effect::MaterialType>>();
	materials.get_mut(&destination_material_handle).expect("Material should exist").set_slot(destination_slot, source_image).expect("Something went wrong idk");
}

pub fn update_effect<Effect: PostProcessingEffect>(
	mut post_processing_materials: ResMut<Assets<Effect::MaterialType>>,
	post_processing_effects: Query<&Effect>,
) {
	for effect in post_processing_effects.iter() {
		if let Some(material)
			= post_processing_materials.get_mut(&effect.get_handle()) {
			effect.update_info(material);
		}
	}
}
