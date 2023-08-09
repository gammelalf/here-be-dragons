use std::fmt::Debug;

use cgmath::{Point3, Vector3, Zero};
use specs::{Component, Join, NullStorage, Read, ReadStorage, System, VecStorage, WriteStorage};

use crate::timer::Delta;

#[derive(Copy, Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Point3<f32>);

#[derive(Copy, Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector3<f32>);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vector3::zero())
    }
}

#[derive(Copy, Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Acceleration(pub Vector3<f32>);

impl Default for Acceleration {
    fn default() -> Self {
        Self(Vector3::zero())
    }
}

#[derive(Copy, Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Planet;

pub struct Mechanics;
impl<'a> System<'a> for Mechanics {
    type SystemData = (
        Read<'a, Delta>,
        ReadStorage<'a, Acceleration>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (delta, acc, mut vel, mut pos): Self::SystemData) {
        for (acc, vel, pos) in ((&acc).maybe(), &mut vel, &mut pos).join() {
            if let Some(acc) = acc {
                vel.0 += acc.0 * delta.as_secs_f32();
            }
            pos.0 += vel.0 * delta.as_secs_f32();
        }
    }
}
