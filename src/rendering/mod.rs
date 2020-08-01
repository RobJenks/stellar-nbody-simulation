#![allow(dead_code)] pub mod colour;
pub mod screenshot;

use std::ops::{Add, Sub, Mul, Div, AddAssign};
use core::iter::Sum;
use ::core::cell::{Ref};
use ::image;
use piston_window::*;
use image::Rgba;
use crate::nbody::nbody_system::NBodySystem;
use crate::state::State;
use crate::core::types::Numeric;
use itertools::Itertools;

pub type BackBuffer = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

const DYNAMIC_BOUNDS : bool = false;

pub struct Renderer {
    bounds: (f64, f64)
}

impl Renderer {
    pub fn new() -> Self {
        Self { bounds: (0.0, 0.0) }
    }

    pub fn prepare_backbuffer(buffer: &mut BackBuffer, draw_size: &[u32; 2], zoom_level: f64, view_origin: [f64; 2]) {
        Renderer::clear_backbuffer(buffer);

        // Prepare backbuffer
        //let bound = u32::min(draw_size[0], draw_size[1]) - 1;
        //(0..bound).for_each(|x| buffer.put_pixel(x,  bound - x, colour::BLUE));
    }

    pub fn perform_rendering<TNum>(&mut self, g: &mut G2d, context: &Context, render_size: (f64, f64), zoom_level: f64, view_origin: [f64; 2],
                                   system: &NBodySystem<TNum>)
        where TNum: Numeric + Add<Output=TNum> + Sub<Output=TNum> + Mul<Output=TNum> + Div<Output=TNum> + AddAssign + Sum {
        piston_window::clear([0.0, 0.0, 0.0, 1.0], g);

        // Get all states to be rendered
        let states = system.get_state_history(1);

        // Determine window bounds across all states
        self.update_bounds(&states);
        let bounds = (self.bounds.0 * 1.1, self.bounds.1 * 1.1);
        let zero_offset = bounds.0 * -1.0;      // e.g. if min bound is -ve, this will be a +ve offset
        let zb_bounds = (0.0, bounds.1 + zero_offset);

        // Perform rendering
        states.iter().for_each(
            |state| state.positions().iter().for_each(|pos| {
                let canvas_pos: (f64, f64) = (
                    ((pos.x().into_f64() + zero_offset) / zb_bounds.1),
                    ((pos.y().into_f64() + zero_offset) / zb_bounds.1));

                let sz = 0.01;
                ellipse_from_to([0.0, 1.0, 0.0, 1.0], [canvas_pos.0 - sz, canvas_pos.1 - sz],
                                [canvas_pos.0 + sz, canvas_pos.1 + sz], context.transform, g);
            }));


        //line_from_to([0.0, 1.0, 0.0, 1.0], 0.1, [0.0, 0.0], [1.0, 1.0], context.transform, g);
    }

    fn update_bounds<TNum>(&mut self, states: &Vec<Ref<State<TNum>>>)
        where TNum: Numeric {

        let new_bounds = Self::determine_bounds(states);
        self.bounds = (
            self.bounds.0.min(new_bounds.0),
            self.bounds.1.max(new_bounds.1)
        );
    }

    fn determine_bounds<TNum>(states: &Vec<Ref<State<TNum>>>) -> (f64, f64)
        where TNum: Numeric {
        if !DYNAMIC_BOUNDS { (-0.6e1, 0.6e1) } else {
            states.iter()
                .map(Renderer::determine_state_bounds)
                .fold1(|(mn, mx), (next_mn, next_mx)| (mn.min(next_mn), mx.max(next_mx)))
                .unwrap_or_else(|| panic!("Failed to determine state bounds for rendering"))
        }
    }

    fn determine_state_bounds<TNum>(state: &Ref<State<TNum>>) -> (f64, f64)
        where TNum: Numeric {
        state.positions().iter()
            .fold((1e200, -1e200), |(mn, mx), pos| (
                mn.min(pos.x().into_f64()).min(pos.y().into_f64()),
                mx.max(pos.x().into_f64()).max(pos.y().into_f64())
            ))
    }

    fn clear_backbuffer(canvas: &mut BackBuffer) {
        canvas.pixels_mut().for_each(|mut p| p.0 = [0, 0, 0, 0]);
    }
}