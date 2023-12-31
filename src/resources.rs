use bevy::{prelude::*, render::{render_resource::{TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, Extent3d}, texture::BevyDefault}};

#[derive(Resource, Debug, Clone, Default)]
pub struct ScoreCounter{
	pub score: u32,
	pub timer: Timer,
}

#[derive(Resource, PartialEq, Debug, Clone, Default)]
pub struct CursorCabinPosition {
	pub world_position: Vec2,
	pub uv_position: Vec2,
}

// Progress values between 0 and 1
#[derive(Resource, PartialEq, Debug, Clone, Default)]
pub struct ProgressBar {
	pub good_progress: f32,
	pub bad_progress: f32,
}

#[derive(Resource, PartialEq, Debug, Clone, Copy)]
pub struct ThoughtSpawnParameters {
	pub far_radius: f32,
	pub close_radius: f32,
	pub despawn_radius: f32,
	pub total_to_spawn: u32,
}

// Texture all the gameplay cameras render to, gets then chewed up by post-proc
#[derive(Resource, Eq, PartialEq, Debug, Clone)]
pub struct MainRenderTexture {
	pub width: u32,
	pub height: u32,
	pub texture: Handle<Image>
}

impl FromWorld for MainRenderTexture {
	fn from_world(world: &mut World) -> Self {
		// Image the main camera renders to
		let (width, height) = (1280, 720);
		Self {
			width, height,
			texture: {
				let size = Extent3d {width, height, ..Default::default()};
				let mut image = Image {
					texture_descriptor: TextureDescriptor {
						label: None,
						size,
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
			}
		}
	}
}
