use bevy::{prelude::*, render::{render_resource::{TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, Extent3d}, texture::BevyDefault}};

#[derive(Resource, PartialEq, Debug, Clone, Default)]
pub struct CursorCabinPosition {
	pub world_position: Vec2,
	pub uv_position: Vec2,
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
