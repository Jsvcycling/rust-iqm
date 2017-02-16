// Based on: https://github.com/tomaka/glium/blob/master/examples/support/camera.rs

extern crate cgmath;

use self::cgmath::{Matrix4, Point3, Vector3};
use self::cgmath::prelude::*;
use glium::glutin::ElementState::*;
use glium::glutin::Event;
use glium::glutin::VirtualKeyCode as KeyCode;

pub struct Camera {
    aspect_ratio: f32,
    position: Point3<f32>,
    direction: Vector3<f32>,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1024.0 / 768.0,
            position: Point3 { x: 0.1, y: 0.0, z: 0.05 },
            direction: Vector3 { x: -1.0, y: 0.0, z: 0.0 },
            
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
        }
    }

    pub fn get_perspective_matrix(&self) -> Matrix4<f32> {
        let fov: f32 = 3.141592 / 2.0;
        let z_far = 1024.0;
        let z_near = 0.01;
        let f = 1.0 / (fov / 2.0).tan();

        let mat = Matrix4::<f32>::new(f / self.aspect_ratio, 0.0, 0.0, 0.0,
                                      0.0, f, 0.0, 0.0,
                                      0.0, 0.0, (z_far + z_near) / (z_far - z_near), 1.0,
                                      0.0, 0.0, -(2.0 * z_far * z_near) / (z_far - z_near), 0.0);

        mat
    }

    // TODO: figure out how to generate this matrix using Matrix4::look_at
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let f = self.direction.normalize();
        let up = Vector3::<f32>::unit_z();
        let s = f.cross(up);
        let s_norm = s.normalize();
        let u = s_norm.cross(f);
        let p = Vector3 {
            x: -self.position.x * s.x - self.position.y * s.y - self.position.z * s.z,
            y: -self.position.x * u.x - self.position.y * u.y - self.position.z * u.z,
            z: -self.position.x * f.x - self.position.z * f.y - self.position.z * f.z
        };
                           
        
        let mat = Matrix4::<f32>::new(s_norm.x, u.x, f.x, 0.0,
                                      s_norm.y, u.y, f.y, 0.0,
                                      s_norm.z, u.z, f.z, 0.0,
                                      p.x, p.y, p.z, 1.0);

        mat
    }

    pub fn update(&mut self) {
        let f = self.direction.normalize();
        let up = Vector3::<f32>::unit_z();
        let s = f.cross(up).normalize();
        let u = s.cross(f);
        
        if self.moving_up {
            self.position.x += u.x * 0.01;
            self.position.y += u.y * 0.01;
            self.position.z += u.z * 0.01;
        }
        
        if self.moving_left {
            self.position.x -= s.x * 0.01;
            self.position.y -= s.y * 0.01;
            self.position.z -= s.z * 0.01;
        }
        
        if self.moving_down {
            self.position.x -= u.x * 0.01;
            self.position.y -= u.y * 0.01;
            self.position.z -= u.z * 0.01;
        }
        
        if self.moving_right {
            self.position.x += s.x * 0.01;
            self.position.y += s.y * 0.01;
            self.position.z += s.z * 0.01;
        }
        
        if self.moving_forward {
            self.position.x += f.x * 0.01;
            self.position.y += f.y * 0.01;
            self.position.z += f.z * 0.01;
        }
        
        if self.moving_backward {
            self.position.x -= f.x * 0.01;
            self.position.y -= f.y * 0.01;
            self.position.z -= f.z * 0.01;
        }
    }
        
    pub fn process_input(&mut self, event: &Event) {
        match event {
            &Event::KeyboardInput(Pressed, _, Some(KeyCode::Space)) => {
                self.moving_up = true;
            },
            &Event::KeyboardInput(Released, _, Some(KeyCode::Space)) => {
                self.moving_up = false;
            },
            
            &Event::KeyboardInput(Pressed, _, Some(KeyCode::LControl)) => {
                self.moving_down = true;
            },
            &Event::KeyboardInput(Released, _, Some(KeyCode::LControl)) => {
                self.moving_down = false;
            },
            
            &Event::KeyboardInput(Pressed, _, Some(KeyCode::A)) => {
                self.moving_left = true;
            },
            &Event::KeyboardInput(Released, _, Some(KeyCode::A)) => {
                self.moving_left = false;
            },
            
            &Event::KeyboardInput(Pressed, _, Some(KeyCode::D)) => {
                self.moving_right = true;
            },
            &Event::KeyboardInput(Released, _, Some(KeyCode::D)) => {
                self.moving_right = false;
            },
            
            &Event::KeyboardInput(Pressed, _, Some(KeyCode::W)) => {
                self.moving_forward = true;
            },
            &Event::KeyboardInput(Released, _, Some(KeyCode::W)) => {
                self.moving_forward = false;
            },
            
            &Event::KeyboardInput(Pressed, _, Some(KeyCode::S)) => {
                self.moving_backward = true;
            },
            &Event::KeyboardInput(Released, _, Some(KeyCode::S)) => {
                self.moving_backward = false;
            },
            
            _ => {},
        }
    }
}
