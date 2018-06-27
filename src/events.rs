use glutin::{self, ElementState::Pressed, KeyboardInput, VirtualKeyCode, WindowEvent};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Button {
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ButtonState {
    Pressed,
    Released,
}

pub enum Event {
    Button { button: Button, state: ButtonState },
    CloseWindow,
    GoFullscreen,
    ExitFullscreen,
    WindowResized(u32, u32),
}

pub struct EventsLoop<'a> {
    glutin_events: Option<&'a mut glutin::EventsLoop>,
    alt_held: bool,
    is_fullscreen: bool,
}

impl<'a> EventsLoop<'a> {
    pub fn new(glutin_events: &'a mut glutin::EventsLoop) -> Self {
        Self {
            glutin_events: Some(glutin_events),
            alt_held: false,
            is_fullscreen: false,
        }
    }

    pub fn poll_events<F: FnMut(Event)>(&mut self, mut callback: F) {
        let glutin_events = self.glutin_events.take().unwrap();
        glutin_events.poll_events(|event| {
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
                    } => callback(Event::CloseWindow),
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
                        self.alt_held = state == Pressed
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
                        if self.alt_held {
                            if self.is_fullscreen {
                                callback(Event::ExitFullscreen);
                            } else {
                                callback(Event::GoFullscreen);
                            }
                            self.is_fullscreen = !self.is_fullscreen;
                        }
                    }
                    WindowEvent::Resized(w, h) => callback(Event::WindowResized(w, h)),
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode,
                                state,
                                ..
                            },
                        ..
                    } if (virtual_keycode == Some(VirtualKeyCode::Left)
                        || virtual_keycode == Some(VirtualKeyCode::A)) =>
                    {
                        if state == Pressed {
                            callback(Event::Button {
                                button: Button::Left,
                                state: ButtonState::Pressed,
                            })
                        } else {
                            callback(Event::Button {
                                button: Button::Left,
                                state: ButtonState::Released,
                            })
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode,
                                state,
                                ..
                            },
                        ..
                    } if (virtual_keycode == Some(VirtualKeyCode::Right)
                        || virtual_keycode == Some(VirtualKeyCode::D)) =>
                    {
                        if state == Pressed {
                            callback(Event::Button {
                                button: Button::Right,
                                state: ButtonState::Pressed,
                            })
                        } else {
                            callback(Event::Button {
                                button: Button::Right,
                                state: ButtonState::Released,
                            })
                        }
                    }
                    _ => (),
                }
            }
        });
        self.glutin_events = Some(glutin_events);
    }
}
