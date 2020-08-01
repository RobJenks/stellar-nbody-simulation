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
use crate::core::simulation;
use shader_version::OpenGL;
use crate::entities::system::System;
use std::ops::DerefMut;

fn main() {
    let sys = System::from_file("resources/systems/test-system.json");
    //let state = sys.generate_state::<f64>();

    let mut nbody = nbody::nbody_system::NBodySystem::<f64>::new(&sys, 5);

    // let mut states = (0..5).map(|_| RefCell::new(state.clone())).collect::<Vec<_>>();
    // states.iter_mut().enumerate().for_each(|(i, x)| x.get_mut().set_id(0, format!("entity-{}", i)));
    //
    // for i in 1..5 {
    //     let s0 = states[i-1].borrow();
    //     let mut s1 = states[i].borrow_mut();
    //     println!("{}, {}", s0.id(0), s1.id(0));
    //     nbody.step(1.0, &*s0, s1.deref_mut());
    // }

    for i in 0..8 {
        nbody.step(1.0);

        println!("\nStep {}, current = {}", i, nbody.current_state_index());
        let all = nbody.get_state_history(3);
        all.iter().for_each(|x| println!("{:?}", x));
    }


    let mut simulation = simulation::Simulation::create(
        simulation::BuildOptions {
            gl_version: OpenGL::V4_5,
            use_cache: false
        },
        nbody
    );

    simulation.execute();
}
