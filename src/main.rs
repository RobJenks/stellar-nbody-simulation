mod core;
mod math;
mod nbody;
mod integrator;
mod rendering;
mod entities;
mod io;
mod state;
mod text;
mod util;

use ::core::cell::RefCell;
use fixed::types::{I16F48, I64F64};
use crate::core::simulation;
use shader_version::OpenGL;
use crate::entities::system::System;
use std::ops::DerefMut;

fn main() {
    let sys = System::from_file("resources/systems/test-system.json");
    let mut nbody = nbody::nbody_system::NBodySystem::<f64>::new(&sys, 400);

    let mut simulation = simulation::Simulation::create(
        simulation::BuildOptions {
            gl_version: OpenGL::V4_5,
            use_cache: false
        },
        nbody
    );

    simulation.execute();
}
