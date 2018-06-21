#[macro_use]
extern crate gfx;
extern crate breakout_core;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate rand;

mod colors;
mod gfx_props;

use breakout_core::{Ball, Breakout, BreakoutBuilder, GameObject, Paddle};
use colors::*;
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_props::*;
use glutin::{ElementState::Pressed, GlContext, KeyboardInput, MouseButton, VirtualKeyCode,
             WindowEvent};
use std::time;

fn get_paddle_vertices_and_indices(game: &Breakout) -> (Vec<Vertex>, Vec<u16>) {
    let (mut vs, mut is) = (vec![], vec![]);

    let ((left, top), (right, bottom)) = game.paddle().boundaries();

    vs.extend(&[
        Vertex {
            pos: [right * 2. - 1., bottom * 2. - 1.],
            color: WHITE,
        },
        Vertex {
            pos: [left * 2. - 1., bottom * 2. - 1.],
            color: WHITE,
        },
        Vertex {
            pos: [left * 2. - 1., top * 2. - 1.],
            color: WHITE,
        },
        Vertex {
            pos: [right * 2. - 1., top * 2. - 1.],
            color: WHITE,
        },
    ]);
    is.extend(&[0, 1, 2, 2, 3, 0]);

    (vs, is)
}

fn get_ball_vertices_and_indices(game: &Breakout) -> (Vec<BallVertex>, Vec<u16>) {
    let (mut vs, mut is) = (vec![], vec![]);

    let radius = game.ball().radius();
    let (left, top, right, bottom) = (0., radius * 2., radius * 2., 0.);

    vs.extend(&[
        BallVertex {
            pos: [right * 2., bottom * 2.],
        },
        BallVertex {
            pos: [left * 2., bottom * 2.],
        },
        BallVertex {
            pos: [left * 2., top * 2.],
        },
        BallVertex {
            pos: [right * 2., top * 2.],
        },
    ]);
    is.extend(&[0, 1, 2, 2, 3, 0]);

    (vs, is)
}

fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Breakout".to_string())
        .with_dimensions(800, 800);

    let gl_builder = glutin::ContextBuilder::new();
    // .with_vsync(true);
    let mut events_loop = glutin::EventsLoop::new();
    let (window, mut device, mut factory, main_color, _) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, gl_builder, &events_loop);

    window.set_cursor(glutin::MouseCursor::NoneCursor);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory
        .create_pipeline_simple(
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/paddle.vert")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/paddle.frag")),
            pipe::new(),
        )
        .unwrap();

    let ball_pso = factory
        .create_pipeline_simple(
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/ball.vert")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/ball.frag")),
            ball_pipe::new(),
        )
        .unwrap();

    let mut game = BreakoutBuilder::new()
        .dt(1. / 120.)
        .ball(Ball::new(0.01, (0.5, 0.7), (0., -0.5)))
        .paddle(Paddle::new((0.15, 0.02), (0.5, 0.075)))
        .build();

    let (vertices, indices) = get_paddle_vertices_and_indices(&game);
    let (ball_vertices, ball_indices) = get_ball_vertices_and_indices(&game);

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, &indices[..]);

    let (ball_vertex_buffer, ball_slice) =
        factory.create_vertex_buffer_with_slice(&ball_vertices, &ball_indices[..]);

    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color.clone(),
    };

    let mut ball_data = ball_pipe::Data {
        vbuf: ball_vertex_buffer,
        midpoint: [
            game.ball().location().0 * 2. - 1.,
            game.ball().location().1 * 2. - 1.,
        ],
        color: RED,
        radius: game.ball().radius() * 2.,
        out: main_color.clone(),
    };

    let nanos_per_update = time::Duration::new(0, (1_000_000_000.0f32 / 120.0f32).round() as u32);

    let mut running = true;
    let mut mouse_held = false;
    let mut alt_held = false;
    let mut window_size = (800.0, 800.0);
    let mut is_fullscreen = false;
    let mut last_update = time::Instant::now();
    let mut needs_update = false;
    while running {
        while last_update.elapsed() >= nanos_per_update {
            game.tick();
            last_update += nanos_per_update;
            needs_update = true;
        }

        if needs_update {
            ball_data.midpoint = [
                game.ball().location().0 * 2. - 1.,
                game.ball().location().1 * 2. - 1.,
            ];
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
                        // gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                        // cube.update_ratio(w as f32 / h as f32);
                        window_size = (w as f32, h as f32);
                    }
                    WindowEvent::MouseInput {
                        state,
                        button: MouseButton::Left,
                        ..
                    } => {
                        if state == Pressed {
                            // cube.start_growing();
                            mouse_held = true;
                        } else {
                            // cube.stop_growing();
                            mouse_held = false;
                        }
                    }
                    _ => (),
                }
            }
        });

        encoder.clear(&ball_data.out, CLEAR_COLOR);
        encoder.draw(&ball_slice, &ball_pso, &ball_data);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
