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

fn main() {
    let tmp = state::State::<f64>::new();

    let mut vmod : Vec<i32> = vec![1,2,3,4];
    let vother : Vec<i32> = vec![5,6,7,8];

    println!("{:?}, {:?}", &vmod, &vother);

    vmod.iter_mut().zip(&vother).for_each(|(mut vm, vo)| *vm += vo);

    println!("{:?}, {:?}", &vmod, &vother);


    let mut simulation = simulation::Simulation::create(
        simulation::BuildOptions {
            gl_version: OpenGL::V4_5,
            use_cache: false
        }
    );

    simulation.execute();
}
