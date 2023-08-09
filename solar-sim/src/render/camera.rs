use cgmath::{Deg, EuclideanSpace, InnerSpace, Matrix4, Point3, Rad, Vector3};
use specs::shred::PanicHandler;
use specs::{Read, System, Write};

use crate::control::Controls;
use crate::timer::Delta;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point3::origin(),
            yaw: Rad(0.0),
            pitch: Rad(0.0),
        }
    }
}

impl Camera {
    pub fn direction(&self) -> Vector3<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize()
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(self.position, self.direction(), Vector3::unit_y())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Projection {
    pub aspect: f32,
    pub fovy: Rad<f32>,
    pub znear: f32,
    pub zfar: f32,
}

impl Projection {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: Deg(45.0).into(),
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ControlCamera {
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for ControlCamera {
    fn default() -> Self {
        Self {
            speed: 10.0,
            sensitivity: 1.0,
        }
    }
}

impl<'a> System<'a> for ControlCamera {
    type SystemData = (
        Read<'a, Delta>,
        Write<'a, Controls>,
        Write<'a, Camera, PanicHandler>,
    );

    fn run(&mut self, (delta, mut controls, mut camera): Self::SystemData) {
        let dt = delta.as_secs_f32();

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += forward
            * (controls.is_forward_pressed as u8 as f32
                - controls.is_backward_pressed as u8 as f32)
            * self.speed
            * dt;
        camera.position += right
            * (controls.is_right_pressed as u8 as f32 - controls.is_left_pressed as u8 as f32)
            * self.speed
            * dt;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let direction = camera.direction();
        camera.position += direction * controls.mouse_scroll * self.speed * self.sensitivity * dt;
        controls.mouse_scroll = 0.0;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.y += (controls.is_up_pressed as u8 as f32
            - controls.is_down_pressed as u8 as f32)
            * self.speed
            * dt;

        // Rotate
        camera.yaw += Rad(controls.mouse_dx) * self.sensitivity * dt;
        camera.pitch += Rad(-controls.mouse_dy) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        controls.mouse_dx = 0.0;
        controls.mouse_dy = 0.0;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
            camera.pitch = Rad(SAFE_FRAC_PI_2);
        }
    }
}

const SAFE_FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2 - 0.0001;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
