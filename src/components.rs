use bevy::prelude::*;

pub use crate::thoughts::Thought;

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Player;

#[derive(Component, Default, PartialEq, Debug, Clone, Copy)]
pub struct Interactable {
	pub radius: f32
}

#[derive(Component, Default, PartialEq, Deref, DerefMut, Debug, Clone, Copy)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default, PartialEq, Deref, DerefMut, Debug, Clone, Copy)]
pub struct AngularVelocity(pub Vec3);

#[derive(Component, Default, PartialEq, Deref, DerefMut, Debug, Clone, Copy)]
pub struct VelocityDrag(pub f32);

#[derive(Component, Default, PartialEq, Deref, DerefMut, Debug, Clone, Copy)]
pub struct AngularVelocityDrag(pub f32);

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Hand;

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct GoodBar;

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct BadBar;

#[derive(Component, Default, PartialEq, Debug, Clone, Copy)]
pub struct Curtain{
	pub left: bool,
	pub dir: f32,
}

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct CabinCutsceneDirector;

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct CabinCamera;

#[derive(Component, Eq, PartialEq, Debug, Clone)]
pub struct CabinThought(pub Thought);

#[derive(Component, PartialEq, Debug, Clone, Copy)]
pub struct CabinButton {
	pub button: crate::prelude::ButtonType,
	pub half_extent: Vec2,
}

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct ClearCamera(pub bool);
