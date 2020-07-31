mod core;
mod rendering;
mod text;
mod util;

use crate::core::simulation;
use shader_version::OpenGL;

fn main() {
    let mut simulation = simulation::Simulation::create(
        simulation::BuildOptions {
            gl_version: OpenGL::V4_5,
            use_cache: false
        }
    );

    simulation.execute();
}
