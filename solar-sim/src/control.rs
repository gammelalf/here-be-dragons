use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode};

#[derive(Copy, Clone, Default, Debug)]
pub struct Controls {
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub mouse_dx: f32,
    pub mouse_dy: f32,
    pub mouse_scroll: f32,
}

impl Controls {
    pub fn process_mouse(&mut self, (dx, dy): (f64, f64)) {
        self.mouse_dx = dx as f32;
        self.mouse_dy = dy as f32;
    }

    pub fn process_wheel(&mut self, delta: MouseScrollDelta) {
        self.mouse_scroll = match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => -scroll * 0.5,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => -scroll as f32,
        };
    }

    pub fn process_keyboard(&mut self, input: &KeyboardInput) -> bool {
        let KeyboardInput {
            state,
            virtual_keycode: Some(keycode),
            ..
        } = input else {return false;};

        let is_pressed = *state == ElementState::Pressed;
        match keycode {
            VirtualKeyCode::Space => {
                self.is_up_pressed = is_pressed;
                true
            }
            VirtualKeyCode::LShift => {
                self.is_down_pressed = is_pressed;
                true
            }
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.is_forward_pressed = is_pressed;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.is_left_pressed = is_pressed;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.is_backward_pressed = is_pressed;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.is_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }
}
