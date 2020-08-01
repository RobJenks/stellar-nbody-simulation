extern crate piston_window;
use ::image;
use piston_window::*;
use crate::rendering;
use crate::rendering::BackBuffer;
use crate::text;
use std::cell::{RefCell, Ref, RefMut};
use crate::util::temporal::get_current_timestamp_secs;
use crate::nbody::nbody_system::NBodySystem;

const MOUSE_LEFT: usize = 0;
const MOUSE_RIGHT: usize = 1;
const MOUSE_BUTTON_COUNT: usize = 2;

const SCROLL_SCALING_FACTOR: f64 = 0.1;
const PAN_SCALING_FACTOR: f64 = 1.5;

const MAX_OBJECT_SELECT_DISTANCE_SQ: f64 = 2.0 * 2.0;

pub struct Simulation {
    window: RefCell<PistonWindow>,
    text_manager: RefCell<text::TextManager>,

    draw_size: [u32; 2],
    draw_sizef: [f64; 2],
    window_size: [f64; 2],
    canvas: BackBuffer,

    zoom_level: f64,
    view_origin: [f64; 2],
    cursor_pos: [f64; 2],
    mouse_down_point: [Option<[f64; 2]>; MOUSE_BUTTON_COUNT],

    nbody_system: NBodySystem<f64>
}

impl Simulation {
    pub fn execute(&mut self) {
        let factory: GfxFactory = self.window().factory.clone();
        let mut texture_context = TextureContext { factory, encoder: self.window_mut().factory.create_command_buffer().into() };
        let mut texture: G2dTexture = Texture::from_image(&mut texture_context,&self.canvas, &TextureSettings::new()).unwrap();

        loop {
            self.nbody_system.step(0.008);    // Temporarily within render loop

            let e_next = self.window_mut().next();
            if e_next == None { break; }
            let e = e_next.unwrap();

            match e {
                Event::Input(event, _timestamp) => match event {
                    Input::Resize(args) => {
                        let window_size = self.window().size();
                        self.update_size(&args.draw_size, window_size);

                        let factory: GfxFactory = self.window().factory.clone();
                        texture_context = TextureContext { factory, encoder: self.window_mut().factory.create_command_buffer().into() };
                        texture = Texture::from_image(&mut texture_context,&self.canvas, &TextureSettings::new()).unwrap();

                        // Trigger post-resize update
                    },
                    Input::Button(args) => {
                        match args.button {
                            Button::Keyboard(key) if args.state == ButtonState::Press => self.key_down(&key),
                            Button::Keyboard(key) if args.state == ButtonState::Release => self.key_up(&key),

                            Button::Mouse(button) if args.state == ButtonState::Press => self.mouse_down(&button),
                            Button::Mouse(button) if args.state == ButtonState::Release => self.mouse_up(&button),

                            _ => ()
                        }
                    },
                    Input::Move(args) => {
                        match args {
                            Motion::MouseCursor(cursor) => self.mouse_move(&cursor),
                            Motion::MouseRelative(movement) => self.mouse_move_relative(&movement),
                            Motion::MouseScroll(scroll) => {
                                self.perform_zoom(scroll);
                                self.update_backbuffer();
                            },
                            _ => ()
                        }
                    }
                    _ => ()
                }
                Event::Loop(event) => match event {
                    Loop::Render(_) => {
                        texture.update(&mut texture_context, &self.canvas).unwrap();
                        let zoom_level = self.zoom_level;
                        let view_origin = self.view_origin;
                        let render_size = self.draw_sizef;
                        let window_size = self.window_size;
                        let scaled_size = (self.draw_sizef[0] / self.zoom_level, self.draw_sizef[1] / self.zoom_level);
                        let mut text_manager = self.text_manager.borrow_mut();
                        let glyph_cache = text_manager.glyph_cache();

                        self.window.borrow_mut().draw_2d(&e, |_context: Context, g, device| {
                            // Global transform to a [0.0 1.0] coordinate space, in each axis
                            let context = piston_window::Context::new_abs(render_size[0], render_size[1])
                                .scale(render_size[0], render_size[1]);

                            // Render all window content
                            rendering::perform_rendering(g, &context, scaled_size, zoom_level, view_origin, &self.nbody_system);

                            // Render status text
                            self.render_text_lines(vec![
                                format!("Step {}, Pos[0] = {:?}", self.nbody_system.get_step_count(), self.nbody_system.get_current_state().position(0)).as_str(),
                                format!("Vel[0] = {:?}", self.nbody_system.get_current_state().velocity(0)).as_str()
                            ],
                            &[0.01, 0.90], 0.035, [0.0,1.0,0.0,1.0], 14, glyph_cache, &context, g);

                            // Apply pre-rendered backbuffer target (if not panning the map)
                            if !self.is_mouse_dragging(MOUSE_RIGHT) {
                                texture_context.encoder.flush(device);
                                image(&texture, context.scale(1.0 / texture.get_width() as f64, 1.0 / texture.get_height() as f64).transform, g);
                            }

                            // Draw zoom box if relevant
                            if self.is_mouse_dragging(MOUSE_LEFT) {
                                let rect = self.get_drag_selection(MOUSE_LEFT, &window_size).unwrap_or_else(|| panic!("No drag entities"));
                                rectangle(rendering::colour::COLOUR_SELECTION, rect, context.transform, g);
                            }

                            glyph_cache.factory.encoder.flush(device);
                        });
                    },
                    Loop::AfterRender(_ar) => {
                        self.update_backbuffer();
                    },
                    _ => ()
                },
                _ => ()
            }
        }
    }

    fn key_down(&mut self, _key: &Key) { }

    fn key_up(&mut self, key: &Key) {
        match key {
            Key::Home => self.reset_view(),
            Key::F12 => rendering::screenshot::display_screenshot(),

            _ => ()
        }
    }

    fn mouse_down(&mut self, button: &MouseButton) {
        if let Some(ix) = Simulation::mouse_button_index(button) {
            self.mouse_down_point[ix] = Some(self.cursor_pos.clone());
        }
    }

    fn mouse_up(&mut self, button: &MouseButton) {
        if let Some(ix) = Simulation::mouse_button_index(button) {
            if self.is_mouse_dragging(ix) {
                self.mouse_drag_up(ix)
            } else {
                self.mouse_click(ix, &self.mouse_down_point[ix].unwrap_or_else(|| panic!("No mouse down location")));
            }

            self.mouse_down_point[ix] = None;
        }
    }

    fn mouse_click(&mut self, button_index: usize, location: &[f64; 2]) {
        match button_index {
            MOUSE_LEFT => (),
            MOUSE_RIGHT => (),
            _ => ()
        }
    }

    fn mouse_drag_up(&mut self, button_index: usize) {
        match button_index {
            MOUSE_LEFT => {         // Post-selection drag
                let rect = self.get_drag_selection(MOUSE_LEFT, &self.window_size).unwrap_or_else(|| panic!("No drag entities"));
                self.zoom_to(&rect);
                self.update_backbuffer();
            }
            MOUSE_RIGHT => {        // Post-drag
                self.update_backbuffer();
            },
            _ => ()
        }
    }

    fn mouse_move(&mut self, cursor: &[f64; 2]) {
        self.cursor_pos = *cursor;
    }

    fn mouse_move_relative(&mut self, movement: &[f64; 2]) {
        if self.mouse_is_down(MOUSE_RIGHT) {
            self.pan_view(
                self.adjust_pan_for_map_settings([
                    -(movement[0] / self.draw_sizef[0]),
                    -(movement[1] / self.draw_sizef[1])
                ])
            );
        }
    }

    fn mouse_is_down(&self, button: usize) -> bool {
        self.mouse_down_point[button].is_some()
    }

    fn is_mouse_dragging(&self, button: usize) -> bool {
        self.mouse_down_point[button]
            .and_then(|start| Some(
                (start[0] - self.cursor_pos[0]).abs() + (start[1] - self.cursor_pos[1]).abs() > MAX_OBJECT_SELECT_DISTANCE_SQ
            ))
            .unwrap_or(false)
    }

    fn get_drag_selection(&self, button: usize, window_size: &[f64; 2]) -> Option<[f64; 4]> {
        if self.is_mouse_dragging(button) {
            let (nx, ny) = (|x| x / window_size[0], |y| y / window_size[1]);

            let start = self.mouse_down_point[button].unwrap_or_else(|| panic!("No mouse down start location"));
            Some([nx(start[0]), ny(start[1]), nx(self.cursor_pos[0] - start[0]), ny(self.cursor_pos[1] - start[1])])
        }
        else {
            None
        }
    }

    fn get_unzoomed_position(&self, pos: [f64; 2]) -> [f64; 2] {
        [self.view_origin[0] + (pos[0] / self.zoom_level),
         self.view_origin[1] + (pos[1] / self.zoom_level)]
    }

    fn window(& self) -> Ref<PistonWindow> {
        self.window.borrow()
    }

    fn window_mut(&self) -> RefMut<PistonWindow> {
        self.window.borrow_mut()
    }

    fn reset_view(&mut self) {
        self.view_origin = [0.0, 0.0];
        self.zoom_level = 1.0;
    }

    fn render_text(&self, text: &str, pos: &[f64; 2], colour: [f32; 4], font_size: u32, glyph_cache: &mut Glyphs, context: &Context, g: &mut G2d) {
        piston_window::text::Text::new_color(colour, font_size).draw(
            text,
            glyph_cache,
            &context.draw_state,
            context.transform
                .scale(1.0 / self.draw_sizef[0], 1.0 / self.draw_sizef[1])
                .trans(pos[0] * self.draw_sizef[0], pos[1] * self.draw_sizef[1]),
            g)
            .unwrap_or_else(|e| panic!("Text rendering failed ({:?})", e));
    }

    fn render_text_lines(&self, text: Vec<&str>, pos: &[f64; 2], line_spacing: f64, colour: [f32; 4],
                         font_size: u32, glyph_cache: &mut Glyphs, context: &Context, g: &mut G2d) {
        text.iter()
            .enumerate()
            .for_each(|(i, &line)| self.render_text(line, &[pos[0], pos[1] + line_spacing * i as f64],
                                                    colour, font_size, glyph_cache, context, g));
    }

    fn update_size(&mut self, size: &[u32; 2], window_size: Size) {
        self.draw_size = *size;
        self.draw_sizef = [self.draw_size[0] as f64, self.draw_size[1] as f64];
        self.window_size = [window_size.width, window_size.height];

        self.canvas = image::ImageBuffer::new(self.draw_size[0], self.draw_size[1]);
    }

    fn update_backbuffer(&mut self) {
        rendering::prepare_backbuffer(&mut self.canvas, &self.draw_size, self.zoom_level, self.view_origin);
    }

    #[allow(unused_parens)]
    fn perform_zoom(&mut self, scroll: [f64; 2]) {
        let (_h_scroll, v_scroll) = (scroll[0], scroll[1]);

        // Set new zoom level
        let original_zoom_level = self.zoom_level;
        self.zoom_level += (v_scroll * SCROLL_SCALING_FACTOR);

        // Limit to acceptable bounds
        self.zoom_level = self.zoom_level.max(0.1);

        // Determine pan required to maintain consistent zoom target
        let size = self.window_size;
        let scale_change = (1.0 / self.zoom_level - 1.0 / original_zoom_level);
        let zoom_point = [self.cursor_pos[0] / size[0], self.cursor_pos[1] / size[1]];

        let offset = (-(zoom_point[0] * scale_change), -(zoom_point[1] * scale_change));
        self.view_origin[0] += offset.0;
        self.view_origin[1] += offset.1;
    }

    fn zoom_to(&mut self, rect: &[f64; 4]) {
        let origin = [rect[0], rect[1]];
        let (width, height) = (rect[2] / self.zoom_level, rect[3] / self.zoom_level);

        self.view_origin = self.get_unzoomed_position(origin);
        self.zoom_level = (1.0 / width).min(1.0 / height);
    }

    fn pan_view(&mut self, pan: [f64; 2]) {
        self.view_origin = [
            self.view_origin[0] + pan[0],
            self.view_origin[1] + pan[1]
        ];
    }

    fn adjust_pan_for_map_settings(&self, pan: [f64; 2]) -> [f64; 2] {
        [
            (pan[0] * PAN_SCALING_FACTOR) / self.zoom_level,
            (pan[1] * PAN_SCALING_FACTOR) / self.zoom_level
        ]
    }


    pub fn create(options: BuildOptions, nbody_system: NBodySystem<f64>) -> Self {
        let mut window = Simulation::init_window(&options);
        let text_manager = Simulation::init_text_manager(text::DEFAULT_FONT.to_string(), &mut window);

        let draw_size: [u32; 2] = [window.draw_size().width as u32, window.draw_size().height as u32];
        let draw_sizef: [f64; 2] = [draw_size[0] as f64, draw_size[1] as f64];
        let window_size = [window.size().width, window.size().height];
        let canvas: BackBuffer = image::ImageBuffer::new(draw_size[0], draw_size[1]);

        Self {
            window: RefCell::new(window),
            text_manager: RefCell::new(text_manager),

            draw_size,
            draw_sizef,
            window_size,
            canvas,

            zoom_level: 1.0,
            view_origin: [0.0, 0.0],
            cursor_pos: [0.0, 0.0],
            mouse_down_point: [None; MOUSE_BUTTON_COUNT],

            nbody_system
        }
    }

    fn init_window(options: &BuildOptions) -> PistonWindow {
        let mut window: PistonWindow = WindowSettings::new("simulation", [512; 2])
            .graphics_api(options.gl_version)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|_| panic!("Cannot initialise window"));

        window.set_lazy(false);
        window
    }

    fn init_text_manager(font: String, window: &mut PistonWindow) -> text::TextManager {
        let glyph_cache = window.load_font(font.as_str())
            .unwrap_or_else(|e| panic!("Failed to initialise text manager ({:?})", e));

        text::TextManager::create(font, glyph_cache)
    }

    fn mouse_button_index(button: &MouseButton) -> Option<usize> {
        match button {
            MouseButton::Left => Some(MOUSE_LEFT),
            MouseButton::Right => Some(MOUSE_RIGHT),

            _ => None
        }
    }


}


pub struct BuildOptions {
    pub gl_version: OpenGL,
    pub use_cache: bool
}