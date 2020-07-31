#![allow(dead_code)] pub mod colour;
pub mod screenshot;

use ::image;
use piston_window::*;
use image::Rgba;

pub type BackBuffer = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub fn prepare_backbuffer(buffer: &mut BackBuffer, draw_size: &[u32; 2], zoom_level: f64, view_origin: [f64; 2]) {
    clear_backbuffer(buffer);

    // Prepare backbuffer
    let bound = u32::min(draw_size[0], draw_size[1]) - 1;
    (0..bound).for_each(|x| buffer.put_pixel(x,  bound - x, colour::BLUE));
}

pub fn perform_rendering(g: &mut G2d, context: &Context, render_size: (f64, f64), zoom_level: f64, view_origin: [f64; 2]) {
    piston_window::clear([0.0, 0.0, 0.0, 1.0], g);

    // Perform rendering
    line_from_to([0.0, 1.0, 0.0, 1.0], 0.1, [0.0, 0.0], [1.0, 1.0], context.transform, g);
}

fn clear_backbuffer(canvas: &mut BackBuffer) {
    canvas.pixels_mut().for_each(|mut p| p.0 = [0, 0, 0, 0]);
}
