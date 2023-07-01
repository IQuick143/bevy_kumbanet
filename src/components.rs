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

#[derive(Component)]
pub struct Win(pub Timer);

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Cursor;

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct GoodBar;

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct BadBar;

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

// When there is an entity with priority speaker enabled, music will quieten
#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct PrioritySpeaker;

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct ClearCamera(pub bool);

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct ScoreText(pub bool);
