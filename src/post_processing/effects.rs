mod effect_imports {
	pub use {
		bevy::{
			prelude::*,
			reflect::TypeUuid,
			render::render_resource::{
				AsBindGroup, ShaderRef, ShaderType,
			},
			sprite::Material2d,
		},
		
		super::super::{PostProcessingEffectMaterial, PostProcessingEffect}		
	};
}

pub mod test {
	use super::effect_imports::*;

	#[derive(Component, Default)]
	pub struct Effect {
		material_handle: Handle<Material>,
	}
	
	impl PostProcessingEffect for Effect {
		type MaterialType = Material;

		fn from_handle(handle: Handle<Self::MaterialType>) -> Self {
			Effect { material_handle: handle }
		}
	
		fn update_info(&self, _material: &mut Self::MaterialType) {
			//todo!()
		}
	
		fn get_handle(&self) -> Handle<Self::MaterialType> { self.material_handle.clone() }
	}
	
	/*#[derive(ShaderType, Default, Clone, Copy)]
	struct ASHAPostProcessingMaterialInner {
		valuea: f32,
		valueb: f32,
		valuec: f32,
		valued: f32
	}*/
	
	/// Our custom post processing material
	#[derive(AsBindGroup, TypeUuid, Clone)]
	#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
	pub struct Material {
		#[texture(0)]
		#[sampler(1)]
		source_image: Handle<Image>,
		//#[uniform(2)]
		//settings: ASHAPostProcessingMaterialInner,
	}
	
	impl Material2d for Material {
		fn fragment_shader() -> ShaderRef {
			"shaders/post_processing/test_effect.wgsl".into()
		}
	}
	
	impl PostProcessingEffectMaterial for Material {
		fn new() -> Self {
			Material { 
				source_image: Handle::default(),
			}
		}

		fn n_slots() -> usize { 1 }

		fn set_slot(&mut self, slot: usize, texture: Handle<Image>) -> Result<(),()> {
			if slot > 0 {
				Err(())
			} else {
				self.source_image = texture;
				Ok(())
			}
		}
	}		
}

pub mod flip {
	use super::effect_imports::*;

	#[derive(Component, Default)]
	pub struct Effect {
		material_handle: Handle<Material>,
	}
	
	impl PostProcessingEffect for Effect {
		type MaterialType = Material;

		fn from_handle(handle: Handle<Self::MaterialType>) -> Self {
			Effect { material_handle: handle }
		}
	
		fn update_info(&self, _material: &mut Self::MaterialType) {
			//todo!()
		}
	
		fn get_handle(&self) -> Handle<Self::MaterialType> { self.material_handle.clone() }
	}
	
	/// Our custom post processing material
	#[derive(AsBindGroup, TypeUuid, Clone)]
	#[uuid = "bc2f08eb-a0fb-43f1-a908-548715a597d5"]
	pub struct Material {
		#[texture(0)]
		#[sampler(1)]
		source_image: Handle<Image>,
	}
	
	impl Material2d for Material {
		fn fragment_shader() -> ShaderRef {
			"shaders/post_processing/flip_effect.wgsl".into()
		}
	}
	
	impl PostProcessingEffectMaterial for Material {
		fn new() -> Self {
			Material { 
				source_image: Handle::default(),
			}
		}

		fn n_slots() -> usize { 1 }

		fn set_slot(&mut self, slot: usize, texture: Handle<Image>) -> Result<(),()> {
			if slot > 0 {
				Err(())
			} else {
				self.source_image = texture;
				Ok(())
			}
		}
	}		
}

pub mod jpeg {
	use super::effect_imports::*;

	#[derive(Component, Default)]
	pub struct Encode {
		material_handle: Handle<EncodeMaterial>,
	}
	
	impl PostProcessingEffect for Encode {
		type MaterialType = EncodeMaterial;

		fn from_handle(handle: Handle<Self::MaterialType>) -> Self {
			Encode { material_handle: handle }
		}
	
		fn update_info(&self, _material: &mut Self::MaterialType) {
			//todo!()
		}
	
		fn get_handle(&self) -> Handle<Self::MaterialType> { self.material_handle.clone() }
	}
	
	/// Our custom post processing material
	#[derive(AsBindGroup, TypeUuid, Clone)]
	#[uuid = "1afecafe-a0fb-43f1-a908-543715a597d5"]
	pub struct EncodeMaterial {
		#[texture(0)]
		#[sampler(1)]
		source_image: Handle<Image>,
	}
	
	impl Material2d for EncodeMaterial {
		fn fragment_shader() -> ShaderRef {
			"shaders/post_processing/jpeg_dct.wgsl".into()
		}
	}
	
	impl PostProcessingEffectMaterial for EncodeMaterial {
		fn new() -> Self {
			EncodeMaterial { 
				source_image: Handle::default(),
			}
		}

		fn n_slots() -> usize { 1 }

		fn set_slot(&mut self, slot: usize, texture: Handle<Image>) -> Result<(),()> {
			if slot > 0 {
				Err(())
			} else {
				self.source_image = texture;
				Ok(())
			}
		}
	}

	#[derive(Component, Default)]
	pub struct Decode {
		material_handle: Handle<DecodeMaterial>,
	}

	impl PostProcessingEffect for Decode {
		type MaterialType = DecodeMaterial;

		fn from_handle(handle: Handle<Self::MaterialType>) -> Self {
			Decode { material_handle: handle }
		}

		fn update_info(&self, _material: &mut Self::MaterialType) {
			//todo!()
		}

		fn get_handle(&self) -> Handle<Self::MaterialType> { self.material_handle.clone() }
	}

	/// Our custom post processing material
	#[derive(AsBindGroup, TypeUuid, Clone)]
	#[uuid = "2afecafe-a0fb-43f1-a908-543715a597d5"]
	pub struct DecodeMaterial {
		#[texture(0)]
		#[sampler(1)]
		source_image: Handle<Image>,
	}

	impl Material2d for DecodeMaterial {
		fn fragment_shader() -> ShaderRef {
			"shaders/post_processing/jpeg_idct.wgsl".into()
		}
	}

	impl PostProcessingEffectMaterial for DecodeMaterial {
		fn new() -> Self {
			DecodeMaterial { 
				source_image: Handle::default(),
			}
		}

		fn n_slots() -> usize { 1 }

		fn set_slot(&mut self, slot: usize, texture: Handle<Image>) -> Result<(),()> {
			if slot > 0 {
				Err(())
			} else {
				self.source_image = texture;
				Ok(())
			}
		}
	}
}

pub mod dither {
	use super::effect_imports::*;

	#[derive(Component, Default)]
	pub struct Effect {
		material_handle: Handle<Material>,
	}
	
	impl PostProcessingEffect for Effect {
		type MaterialType = Material;

		fn from_handle(handle: Handle<Self::MaterialType>) -> Self {
			Effect { material_handle: handle }
		}
	
		fn update_info(&self, _material: &mut Self::MaterialType) {
			//todo!()
		}
	
		fn get_handle(&self) -> Handle<Self::MaterialType> { self.material_handle.clone() }
	}
	
	/// Our custom post processing material
	#[derive(AsBindGroup, TypeUuid, Clone)]
	#[uuid = "bd2fa9e5-a1fb-43f1-a908-548715a597d5"]
	pub struct Material {
		#[texture(0)]
		#[sampler(1)]
		source_image: Handle<Image>,
	}
	
	impl Material2d for Material {
		fn fragment_shader() -> ShaderRef {
			"shaders/post_processing/dither_effect.wgsl".into()
		}
	}
	
	impl PostProcessingEffectMaterial for Material {
		fn new() -> Self {
			Material { 
				source_image: Handle::default(),
			}
		}

		fn n_slots() -> usize { 1 }

		fn set_slot(&mut self, slot: usize, texture: Handle<Image>) -> Result<(),()> {
			if slot > 0 {
				Err(())
			} else {
				self.source_image = texture;
				Ok(())
			}
		}
	}		
}

pub mod feedback {
	use super::effect_imports::*;

	#[derive(Component)]
	pub struct Effect {
		material_handle: Handle<Material>,
		pub time: f32,
		pub alpha: f32,
		pub beta: f32,
	}

	impl Default for Effect {
		fn default() -> Self {
			Self {
				material_handle: Default::default(),
				time: 0.0,
				alpha: 0.42,
				beta: 0.25,
			}
		}
	}
	
	impl PostProcessingEffect for Effect {
		type MaterialType = Material;

		fn from_handle(handle: Handle<Self::MaterialType>) -> Self {
			Effect { material_handle: handle, ..default() }
		}
	
		fn update_info(&self, material: &mut Self::MaterialType) {
			material.data.time = self.time;
			material.data.alpha = self.alpha;
			material.data.beta = self.beta;
		}
	
		fn get_handle(&self) -> Handle<Self::MaterialType> { self.material_handle.clone() }
	}

	#[derive(ShaderType, Default, Clone, Copy)]
	struct MaterialInner {
		time: f32,
		alpha: f32,
		beta: f32,
		valued: f32
	}
	
	/// Our custom post processing material
	#[derive(AsBindGroup, TypeUuid, Clone)]
	#[uuid = "cd2fa9e5-aefb-4389-a908-548715a597d5"]
	pub struct Material {
		#[texture(0)]
		#[sampler(1)]
		source_image: Handle<Image>,
		#[texture(2)]
		#[sampler(3)]
		feedback_image: Handle<Image>,
		#[uniform(4)]
		data: MaterialInner,
	}
	
	impl Material2d for Material {
		fn fragment_shader() -> ShaderRef {
			"shaders/post_processing/feedback.wgsl".into()
		}
	}
	
	impl PostProcessingEffectMaterial for Material {
		fn new() -> Self {
			Material { 
				source_image: Handle::default(),
				feedback_image: Handle::default(),
				data: MaterialInner::default()
			}
		}

		fn n_slots() -> usize { 2 }

		fn set_slot(&mut self, slot: usize, texture: Handle<Image>) -> Result<(),()> {
			if slot > 1 {
				Err(())
			} else if slot == 0 {
				self.source_image = texture;
				Ok(())
			} else {
				self.feedback_image = texture;
				Ok(())
			}
		}
	}		
}
