#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate rand;

mod colors;
mod cursor;
mod gfx_props;
mod pseudocube;
mod square;

use colors::*;
use gfx::texture::Mipmap;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_props::*;
use glutin::{ElementState::Pressed, GlContext, KeyboardInput, MouseButton, VirtualKeyCode,
             WindowEvent};
use pseudocube::Pseudocube;

fn load_texture<F, R>(factory: &mut F, path: &str) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
where
    F: gfx::Factory<R>,
    R: gfx::Resources,
{
    let img = image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory
        .create_texture_immutable_u8::<ColorFormat>(kind, Mipmap::Provided, &[&img])
        .unwrap();
    view
}

fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Breakout".to_string())
        .with_dimensions(800, 800);

    let gl_builder = glutin::ContextBuilder::new().with_vsync(true);
    let mut events_loop = glutin::EventsLoop::new();
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, gl_builder, &events_loop);

    window.set_cursor(glutin::MouseCursor::NoneCursor);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory
        .create_pipeline_simple(
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/triangle.glslv"
            )),
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/shaders/triangle.glslf"
            )),
            pipe::new(),
        )
        .unwrap();

    let mut cube = Pseudocube::new();

    let (vertices, indices) = cube.get_vertices_indices();

    let (vertex_buffer, mut slice) =
        factory.create_vertex_buffer_with_slice(&vertices, &indices[..]);

    let texture = load_texture(&mut factory, "assets/awesome.jpg");
    let sampler = factory.create_sampler_linear();

    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        awesome: (texture, sampler),
        switch: 0,
        out: main_color,
    };

    let mut running = true;
    let mut needs_update = false;
    let mut mouse_held = false;
    let mut alt_held = false;
    let mut window_size = (800.0, 800.0);
    let mut is_fullscreen = false;
    while running {
        if needs_update {
            let (vs, is) = cube.get_vertices_indices();
            let (vbuf, sl) = factory.create_vertex_buffer_with_slice(&vs, &is[..]);

            data.vbuf = vbuf;
            slice = sl;

            needs_update = false;
        }

        // fetch events
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => running = false,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Space),
                                state: Pressed,
                                ..
                            },
                        ..
                    } => data.switch = 1 - data.switch,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode,
                                state,
                                ..
                            },
                        ..
                    } if (virtual_keycode == Some(VirtualKeyCode::LAlt)
                        || virtual_keycode == Some(VirtualKeyCode::RAlt)
                        || virtual_keycode == Some(VirtualKeyCode::LMenu)
                        || virtual_keycode == Some(VirtualKeyCode::RMenu)) =>
                    {
                        alt_held = state == Pressed;
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Return),
                                state: Pressed,
                                ..
                            },
                        ..
                    } => {
                        if alt_held {
                            if is_fullscreen {
                                let current = window.get_current_monitor();
                                window.set_fullscreen(Some(current));
                            } else {
                                window.set_fullscreen(None);
                            }
                            is_fullscreen = !is_fullscreen;
                        }
                    }
                    WindowEvent::Resized(w, h) => {
                        gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                        cube.update_ratio(w as f32 / h as f32);
                        window_size = (w as f32, h as f32);
                        needs_update = true;
                    }
                    WindowEvent::CursorMoved {
                        position: (x, y), ..
                    } => {
                        cube.update_cursor_position(
                            x as f32 / window_size.0,
                            y as f32 / window_size.1,
                        );
                        needs_update = true;
                    }
                    WindowEvent::MouseInput {
                        state,
                        button: MouseButton::Left,
                        ..
                    } => {
                        if state == Pressed {
                            cube.start_growing();
                            mouse_held = true;
                        } else {
                            cube.stop_growing();
                            mouse_held = false;
                        }
                    }
                    _ => (),
                }
            }
        });

        if mouse_held {
            needs_update = true;
        }

        cube.tick();

        // cube.add_square(
        //     rand::random::<f32>() * 2.0 - 1.0,
        //     rand::random::<f32>() * 2.0 - 1.0,
        //     rand::random::<f32>() * 2.0,
        //     [rand::random(), rand::random(), rand::random()],
        // );
        // needs_update = true;

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
