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

pub fn prepare_backbuffer(buffer: &mut BackBuffer, draw_size: &[u32; 2], zoom_level: f64, view_origin: [f64; 2]) {
    clear_backbuffer(buffer);

    // Prepare backbuffer
    //let bound = u32::min(draw_size[0], draw_size[1]) - 1;
    //(0..bound).for_each(|x| buffer.put_pixel(x,  bound - x, colour::BLUE));
}

pub fn perform_rendering<TNum>(g: &mut G2d, context: &Context, render_size: (f64, f64), zoom_level: f64, view_origin: [f64; 2],
                         system: &NBodySystem<TNum>)
    where TNum: Numeric + Add<Output = TNum> + Sub<Output = TNum> + Mul<Output = TNum> + Div<Output = TNum> + AddAssign + Sum + Into<f64> {

    piston_window::clear([0.0, 0.0, 0.0, 1.0], g);

    // Get all states to be rendered
    let states = system.get_state_history(1);

    // Determine window bounds across all states
    let bounds = determine_bounds(&states);
    let zero_offset = bounds.0 * -1.0;      // e.g. if min bound is -ve, this will be a +ve offset
    let zb_bounds = (0.0, bounds.1 + zero_offset);

    // Perform rendering
    states.iter().for_each(
        |state| state.positions().iter().for_each(|pos| {
            let canvas_pos: (f64, f64) = (
                ((pos.x().into() + zero_offset) / zb_bounds.1),
                ((pos.z().into() + zero_offset) / zb_bounds.1));

            let sz = 0.01;
            ellipse_from_to([0.0, 1.0, 0.0, 1.0], [canvas_pos.0-sz, canvas_pos.1-sz],
                            [canvas_pos.0+sz,canvas_pos.1+sz], context.transform, g);
        }));
    //line_from_to([0.0, 1.0, 0.0, 1.0], 0.1, [0.0, 0.0], [1.0, 1.0], context.transform, g);
}

fn determine_bounds<TNum>(states: &Vec<Ref<State<TNum>>>) -> (f64, f64)
    where TNum: Numeric + Into<f64> {

    states.iter()
        .map(determine_state_bounds)
        .fold1(|(mn,mx),(next_mn,next_mx)| (mn,mx))
        .unwrap_or_else(|| panic!("Failed to determine state bounds for rendering"))
}

fn determine_state_bounds<TNum>(state: &Ref<State<TNum>>) -> (f64, f64)
    where TNum: Numeric + Into<f64> {

    state.positions().iter()
        .fold((1e200 ,-1e200), |(mn, mx), pos| (
            mn.min(pos.x().into()).min(pos.z().into()),
            mx.max(pos.x().into()).max(pos.z().into())
        ))
}

fn clear_backbuffer(canvas: &mut BackBuffer) {
    canvas.pixels_mut().for_each(|mut p| p.0 = [0, 0, 0, 0]);
}
