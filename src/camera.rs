// Based on: https://github.com/tomaka/glium/blob/master/examples/support/camera.rs

extern crate glutin;

use glutin::ElementState::{Pressed, Released};
use glutin::Event;
use glutin::VirtualKeyCode as KeyCode;

struct Camera {
    aspect_ratio: f32,
    position: (f32, f32, f32),
    direction: (f32, f32, f32),

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
            position: (0.1, 0.1, 1.0),
            direction: (0.0, 0.0, -1.0),
            
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
        }
    }

    pub fn get_perspective_matrix(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let z_far = 1024.0;
        let z_near = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // NB: This is in a column-major format (e.g. LoC = column).
        [
            [ f / self.aspect_ratio, 0.0, 0.0, 0.0 ],
            [ 0.0, f, 0.0, 0.0 ],
            [ 0.0, 0.0, (z_far + z_near) / (z_far - z_near), 1.0 ],
            [ 0.0, 0.0, -(2.0 * z_far * z_near) / (z_far - z_near), 0.0 ],
        ]
    }

    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        // Normalize the direction
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        // Normalize the s vector
        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s_norm.1 * f.2 - s_norm.2 * f.1,
                 s_norm.2 * f.0 - s_norm.0 * f.2,
                 s_norm.0 * f.1 - s_norm.1 * f.0);

        let p = (-self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
                 -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
                 -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2);

        // NB: This is in a column-major format (e.g. LoC = column).
        [
            [ s_norm.0, u.0, f.0, 0.0 ],
            [ s_norm.1, u.1, f.1, 0.0 ],
            [ s_norm.2, u.2, f.2, 0.0 ],
            [ p.0, p.1, p.2, 1.0 ],
        ]
    }

    pub fn update(&mut self) {
        // Normalize the direction
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);
        
        // Normalize the s vector
        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };
        
        let u = (s.1 * f.2 - s.2 * f.1,
                 s.2 * f.0 - s.0 * f.2,
                 s.0 * f.1 - s.1 * f.0);
        
        if self.moving_up {
            self.position.0 += u.0 * 0.01;
            self.position.1 += u.1 * 0.01;
            self.position.2 += u.2 * 0.01;
        }
        
        if self.moving_left {
            self.position.0 -= s.0 * 0.01;
            self.position.1 -= s.1 * 0.01;
            self.position.2 -= s.2 * 0.01;
        }
        
        if self.moving_down {
            self.position.0 -= u.0 * 0.01;
            self.position.1 -= u.1 * 0.01;
            self.position.2 -= u.2 * 0.01;
        }
        
        if self.moving_right {
            self.position.0 += s.0 * 0.01;
            self.position.1 += s.1 * 0.01;
            self.position.2 += s.2 * 0.01;
        }
        
        if self.moving_forward {
            self.position.0 += f.0 * 0.01;
            self.position.1 += f.1 * 0.01;
            self.position.2 += f.2 * 0.01;
        }
        
        if self.moving_backward {
            self.position.0 -= f.0 * 0.01;
            self.position.1 -= f.1 * 0.01;
            self.position.2 -= f.2 * 0.01;
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
        }
    }
}
