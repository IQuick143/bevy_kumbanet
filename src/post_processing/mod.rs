pub mod effects;
pub mod setup;

use std::marker::PhantomData;
use std::hash::Hash;

use bevy::{
	prelude::{Handle, Image, SystemSet, Component, Entity, Plugin, IntoSystemConfig, in_state},
	sprite::{Material2d, Material2dPlugin},
	render::render_resource::AsBindGroup
};

pub use setup::{VFXPlugin, spawn_effect, link_effect, update_effect};

use crate::GameState;

// System set for all the systems which edit an effect
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct VFXChangeSystemSet;

#[derive(Default)]
pub struct EffectPlugin<T: PostProcessingEffect>(PhantomData<T>);

impl<T: PostProcessingEffect>  Plugin for EffectPlugin<T>
where <T::MaterialType as AsBindGroup>::Data: PartialEq + Eq + Hash + Clone {
    fn build(&self, app: &mut bevy::prelude::App) {
		app
		.add_plugin(Material2dPlugin::<T::MaterialType>::default())
		.add_system(update_effect::<T>.after(VFXChangeSystemSet).run_if(in_state(GameState::Game)));
    }
}

pub trait PostProcessingEffectMaterial : Material2d {
	fn new() -> Self;
	fn n_slots() -> usize;
	fn set_slot(&mut self, slot: usize, texture: Handle<Image>) -> Result<(),()>;
}

pub trait PostProcessingEffect : Component + Default {
	type MaterialType : PostProcessingEffectMaterial;

	fn from_handle(handle: Handle<Self::MaterialType>) -> Self;
	fn update_info(&self, material: &mut Self::MaterialType);
	fn get_handle(&self) -> Handle<Self::MaterialType>;
}

#[derive(Component, PartialEq, Eq, Debug)]
pub struct EffectAssociatedCameraID(Entity);

#[derive(Component, PartialEq, Eq, Debug)]
pub struct EffectCamera;
