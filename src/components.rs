use bevy::prelude::*;

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
pub struct Thought;
