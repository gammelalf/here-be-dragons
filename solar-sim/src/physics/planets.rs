//! Populate the world with our planets based on some data copied from wikipedia

use cgmath::{Point3, Vector3, Zero};
use specs::{Builder, World, WorldExt};

use crate::physics::{Acceleration, Mass, Planet, Position, Velocity};

/// Populate the world with our planets
pub fn build_planets(world: &mut World) {
    world.register::<Mass>();
    for planet in &PLANETS[..] {
        world
            .create_entity()
            .with(Planet)
            .with(Position(planet.position))
            .with(Velocity(planet.velocity))
            .with(Acceleration(Vector3::zero()))
            .with(Mass(planet.mass))
            .build();
    }
}

/// Data of our planets copied from wikipedia
const PLANETS: [PlanetData; 9] = [
    PlanetData {
        name: "sun",
        position: Point3::new(0.0, 0.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 0.0),
        mass: 1.989e30,
    },
    PlanetData {
        name: "mercury",
        position: Point3::new(57.909e9, 0.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 47.36e3),
        mass: 0.33011e24,
    },
    PlanetData {
        name: "venus",
        position: Point3::new(108.209e9, 0.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 35.02e3),
        mass: 4.8675e24,
    },
    PlanetData {
        name: "earth",
        position: Point3::new(149.596e9, 0.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 29.78e3),
        mass: 5.9724e24,
    },
    PlanetData {
        name: "mars",
        position: Point3::new(227.923e9, 0.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 24.07e3),
        mass: 0.64171e24,
    },
    PlanetData {
        name: "jupiter",
        position: Point3::new(778.570e9, 0.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 13e3),
        mass: 1898.19e24,
    },
    PlanetData {
        name: "saturn",
        position: Point3::new(1433.529e9, 0.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 9.68e3),
        mass: 568.34e24,
    },
    PlanetData {
        name: "uranus",
        position: Point3::new(2872.463e9, 0.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 6.80e3),
        mass: 86.813e24,
    },
    PlanetData {
        name: "neptune",
        position: Point3::new(4495.060e9, 0.0, 0.0),
        velocity: Vector3::new(0.0, 0.0, 5.43e3),
        mass: 102.413e24,
    },
];

struct PlanetData {
    name: &'static str,
    position: Point3<f32>,
    velocity: Vector3<f32>,
    mass: f32,
}
