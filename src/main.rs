#[macro_use]
extern crate gfx;
extern crate breakout_core;
extern crate gfx_glyph;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate rand;

mod colors;
mod events;
mod gfx_props;

use breakout_core::{Ball, Breakout, BreakoutBuilder, GameObject, Paddle};
use colors::*;
use events::{Button, ButtonState::Pressed, Event};
use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_glyph::{GlyphBrushBuilder, Section};
use gfx_props::*;
use glutin::GlContext;
use std::time::{Duration, Instant};

fn get_paddle_vertices_and_indices(game: &Breakout) -> (Vec<PaddleVertex>, Vec<u16>) {
    let (mut vs, mut is) = (vec![], vec![]);

    let (length, height) = game.paddle().dimensions();

    let (left, top, right, bottom) = (0., height, length, 0.);

    vs.extend(&[
        PaddleVertex {
            pos: [right * 2., bottom * 2.],
            color: WHITE,
        },
        PaddleVertex {
            pos: [left * 2., bottom * 2.],
            color: WHITE,
        },
        PaddleVertex {
            pos: [left * 2., top * 2.],
            color: WHITE,
        },
        PaddleVertex {
            pos: [right * 2., top * 2.],
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

    // For some reason the game looks super framey with VSync on, so disabling for now...
    let vsync = false;

    let gl_builder = glutin::ContextBuilder::new().with_vsync(vsync);
    let mut events_loop = glutin::EventsLoop::new();
    let (window, mut device, mut factory, main_color, depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, gl_builder, &events_loop);

    let mut events_loop = events::EventsLoop::new(&mut events_loop);

    window.set_cursor(glutin::MouseCursor::NoneCursor);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut glyph_brush = GlyphBrushBuilder::using_font_bytes(
        &include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/DroidSansMono.ttf"
        ))[..],
    ).build(factory.clone());

    let pso = factory
        .create_pipeline_simple(
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/paddle.vert")),
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/paddle.frag")),
            paddle_pipe::new(),
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
        .dt(1. / 960.)
        .ball(Ball::new(0.02, (0.5, 0.7), (0., -0.5)))
        .paddle(Paddle::new((0.15, 0.02), (0.5, 0.075)))
        .build();

    let (vertices, indices) = get_paddle_vertices_and_indices(&game);
    let (ball_vertices, ball_indices) = get_ball_vertices_and_indices(&game);

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, &indices[..]);

    let (ball_vertex_buffer, ball_slice) =
        factory.create_vertex_buffer_with_slice(&ball_vertices, &ball_indices[..]);

    let mut paddle_data = {
        let ((left, _), (_, bottom)) = game.paddle().boundaries();
        paddle_pipe::Data {
            vbuf: vertex_buffer,
            corner: [left * 2. - 1., bottom * 2. - 1.],
            out: main_color.clone(),
        }
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

    let nanos_per_update = Duration::from_secs(1) / 960;

    let mut last_fps_update = Instant::now();
    let mut frame_count: i32 = 0;
    let mut fps_text = "FPS: -".to_owned();
    let vsync_text = format!("vsync: {}", if vsync { "ON" } else { "OFF" });

    let mut running = true;
    let mut window_size = (800.0, 800.0);
    let mut last_update = Instant::now();
    let mut needs_update = false;
    while running {
        // fetch events
        events_loop.poll_events(|event| {
            match event {
                Event::CloseWindow => running = false,
                Event::GoFullscreen => {
                    let current = window.get_current_monitor();
                    window.set_fullscreen(Some(current));
                }
                Event::ExitFullscreen => {
                    window.set_fullscreen(None);
                }
                Event::WindowResized(w, h) => {
                    // gfx_window_glutin::update_views(&window, &mut data.out, &mut main_depth);
                    // cube.update_ratio(w as f32 / h as f32);
                    window_size = (w as f32, h as f32);
                }
                Event::Button { button, state }
                    if button == Button::Left || button == Button::Right =>
                {
                    let speed = 0.40;
                    let velocity = speed * if button == Button::Left { -1. } else { 1. };

                    if state == Pressed {
                        game.paddle_mut().set_velocity((velocity, 0.));
                    } else {
                        game.paddle_mut().set_velocity((0., 0.));
                    }
                }
                _ => (),
            }
        });

        let mut max_fall_behind = Duration::from_secs(1) / 15;
        while last_update.elapsed() >= nanos_per_update {
            game.tick();
            last_update += nanos_per_update;
            needs_update = true;
            match max_fall_behind.checked_sub(nanos_per_update) {
                Some(fall_behind) => max_fall_behind = fall_behind,
                None => {
                    // if we fall to 15 frames per second, slow down the simulation.
                    last_update = Instant::now();
                    break;
                }
            };
        }

        if needs_update {
            {
                let ((left, _), (_, bottom)) = game.paddle().boundaries();
                paddle_data.corner = [left * 2. - 1., bottom * 2. - 1.];
            }
            ball_data.midpoint = [
                game.ball().location().0 * 2. - 1.,
                game.ball().location().1 * 2. - 1.,
            ];
        }

        encoder.clear(&ball_data.out, CLEAR_COLOR);
        encoder.draw(&ball_slice, &ball_pso, &ball_data);
        encoder.draw(&slice, &pso, &paddle_data);

        frame_count += 1;

        if last_fps_update.elapsed() > Duration::from_secs(1) {
            fps_text = format!("FPS: {}", frame_count);
            frame_count = 0;
            last_fps_update = Instant::now();
        }

        let text = format!("{}\n{}", fps_text, vsync_text);

        let section = Section {
            text: &text,
            color: [1., 1., 1., 1.],
            ..Section::default()
        };

        glyph_brush.queue(section);

        glyph_brush
            .draw_queued(&mut encoder, &main_color, &depth)
            .unwrap();

        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
