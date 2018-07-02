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

#[derive(Copy, Clone)]
enum DirectionButton {
    Left = 0,
    Right = 1,
}

fn get_button_from_direction(dir: DirectionButton) -> Button {
    match dir {
        DirectionButton::Left => Button::Left,
        DirectionButton::Right => Button::Right,
    }
}

fn get_direction_from_integer(num: u8) -> DirectionButton {
    match num {
        0 => DirectionButton::Left,
        1 => DirectionButton::Right,
        x => panic!("Did not expect num {}", x),
    }
}

fn is_left_keycode(virtual_keycode: &Option<VirtualKeyCode>) -> bool {
    (*virtual_keycode == Some(VirtualKeyCode::Left) || *virtual_keycode == Some(VirtualKeyCode::A))
}

fn is_right_keycode(virtual_keycode: &Option<VirtualKeyCode>) -> bool {
    (*virtual_keycode == Some(VirtualKeyCode::Right) || *virtual_keycode == Some(VirtualKeyCode::D))
}

fn get_direction_button(virtual_keycode: &Option<VirtualKeyCode>) -> Option<DirectionButton> {
    if is_left_keycode(virtual_keycode) {
        Some(DirectionButton::Left)
    } else if is_right_keycode(virtual_keycode) {
        Some(DirectionButton::Right)
    } else {
        None
    }
}

pub struct EventsLoop<'a> {
    glutin_events: Option<&'a mut glutin::EventsLoop>,
    alt_held: bool,
    is_fullscreen: bool,
    direction_state: [bool; 2],
}

impl<'a> EventsLoop<'a> {
    pub fn new(glutin_events: &'a mut glutin::EventsLoop) -> Self {
        Self {
            glutin_events: Some(glutin_events),
            alt_held: false,
            is_fullscreen: false,
            direction_state: [false, false],
        }
    }

    fn handle_control(
        &mut self,
        button: DirectionButton,
        state: bool,
        callback: &mut impl FnMut(Event),
    ) {
        match state {
            true => {
                if self.direction_state[1 - (button as usize)] {
                    callback(Event::Button {
                        button: get_button_from_direction(get_direction_from_integer(
                            1 - (button as u8),
                        )),
                        state: ButtonState::Released,
                    });

                    self.direction_state[1 - (button as usize)] = false;
                }

                if !self.direction_state[button as usize] {
                    callback(Event::Button {
                        button: get_button_from_direction(button),
                        state: ButtonState::Pressed,
                    });
                    self.direction_state[button as usize] = true;
                }
            }
            false => {
                if self.direction_state[button as usize] {
                    callback(Event::Button {
                        button: get_button_from_direction(button),
                        state: ButtonState::Released,
                    });
                    self.direction_state[button as usize] = false;
                }
            }
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
                    } if get_direction_button(&virtual_keycode).is_some() =>
                    {
                        let dir = get_direction_button(&virtual_keycode).unwrap();
                        self.handle_control(dir, state == Pressed, &mut callback);
                    }
                    // WindowEvent::KeyboardInput {
                    //     input:
                    //         KeyboardInput {
                    //             virtual_keycode,
                    //             state,
                    //             ..
                    //         },
                    //     ..
                    // } if (virtual_keycode == Some(VirtualKeyCode::Left)
                    //     || virtual_keycode == Some(VirtualKeyCode::A)) =>
                    // {
                    //     if state == Pressed {
                    //         callback(Event::Button {
                    //             button: Button::Left,
                    //             state: ButtonState::Pressed,
                    //         })
                    //     } else {
                    //         callback(Event::Button {
                    //             button: Button::Left,
                    //             state: ButtonState::Released,
                    //         })
                    //     }
                    // }
                    // WindowEvent::KeyboardInput {
                    //     input:
                    //         KeyboardInput {
                    //             virtual_keycode,
                    //             state,
                    //             ..
                    //         },
                    //     ..
                    // } if (virtual_keycode == Some(VirtualKeyCode::Right)
                    //     || virtual_keycode == Some(VirtualKeyCode::D)) =>
                    // {
                    //     if state == Pressed {
                    //         callback(Event::Button {
                    //             button: Button::Right,
                    //             state: ButtonState::Pressed,
                    //         })
                    //     } else {
                    //         callback(Event::Button {
                    //             button: Button::Right,
                    //             state: ButtonState::Released,
                    //         })
                    //     }
                    // }
                    _ => (),
                }
            }
        });
        self.glutin_events = Some(glutin_events);
    }
}
