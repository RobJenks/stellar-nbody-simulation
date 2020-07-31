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

use crate::core::simulation;
use shader_version::OpenGL;
use crate::entities::system::System;

fn main() {
    let sys = System::from_file("resources/systems/test-system.json");
    let state = sys.generate_state::<f64>();

    println!("{:?}", sys);

    let mut simulation = simulation::Simulation::create(
        simulation::BuildOptions {
            gl_version: OpenGL::V4_5,
            use_cache: false
        }
    );

    simulation.execute();
}
