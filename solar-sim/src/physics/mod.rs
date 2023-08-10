//! Collection of components and system to simulate physics

pub mod planets;

use std::fmt::Debug;

use cgmath::{InnerSpace, Point3, Vector3, Zero};
use specs::{
    Component, Entities, Join, NullStorage, Read, ReadStorage, System, VecStorage, WriteStorage,
};

use crate::timer::Delta;

/// Position component
#[derive(Copy, Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Point3<f32>);

/// Velocity component
///
/// Requires a [`Position`] component to affect anything
#[derive(Copy, Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector3<f32>);

/// Acceleration component
///
/// Requires a [`Velocity`] component to affect anything
#[derive(Copy, Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Acceleration(pub Vector3<f32>);

/// Mass component
///
/// Requires a [`Position`] component to affect others
/// Requires a [`Position`] and an [`Acceleration`] component to be affected by others
#[derive(Copy, Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Mass(pub f32);

/// Marker component for things to render
#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Planet;

/// Simulation speed resource
///
/// Defaults to [`1000.0`](SimSpeed::default)
#[derive(Copy, Clone, Debug)]
pub struct SimSpeed(pub f32);

impl Default for SimSpeed {
    fn default() -> Self {
        Self(1000.0)
    }
}

/// System for **basic** mechanics
///
/// Applies [`Acceleration`] to [`Velocity`]
/// and [`Velocity`] to [`Position`].
pub struct Mechanics;
impl<'a> System<'a> for Mechanics {
    type SystemData = (
        Read<'a, SimSpeed>,
        Read<'a, Delta>,
        ReadStorage<'a, Acceleration>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (speed, delta, acc, mut vel, mut pos): Self::SystemData) {
        for (acc, vel) in (&acc, &mut vel).join() {
            vel.0 += acc.0 * delta.as_secs_f32() * speed.0;
        }
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.0 += vel.0 * delta.as_secs_f32() * speed.0;
        }
    }
}

/// System for gravity
pub struct Gravity;
impl<'a> System<'a> for Gravity {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Mass>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Acceleration>,
    );

    fn run(&mut self, (ent, mass, pos, mut acc): Self::SystemData) {
        for (this, _, this_pos, this_acc) in (&ent, &mass, &pos, &mut acc).join() {
            this_acc.0 = Vector3::zero();
            for (other, other_mass, other_pos) in (&ent, &mass, &pos).join() {
                if this != other {
                    let r = other_pos.0 - this_pos.0;
                    let acc = G * other_mass.0 / r.magnitude().powi(2);
                    this_acc.0 += acc * r.normalize();
                }
            }
        }
    }
}

/// Gravitational constant
pub const G: f32 = 6.67e-11;
