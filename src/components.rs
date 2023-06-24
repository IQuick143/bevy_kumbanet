use bevy::prelude::*;

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Player;

#[derive(Component, Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct PlayerVelocity;

#[derive(Component, Default, PartialEq, Deref, DerefMut, Debug, Clone, Copy)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default, PartialEq, Deref, DerefMut, Debug, Clone, Copy)]
pub struct AngularVelocity(pub Vec3);

#[derive(Component, Default, PartialEq, Deref, DerefMut, Debug, Clone, Copy)]
pub struct VelocityDrag(pub f32);

#[derive(Component, Default, PartialEq, Deref, DerefMut, Debug, Clone, Copy)]
pub struct AngularVelocityDrag(pub f32);