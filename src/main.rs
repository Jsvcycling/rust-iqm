#[macro_use]
extern crate glium;
extern crate byteorder;

use std::io::prelude::*;
use std::fs::File;
use std::time::{Duration, Instant};
use std::process;
use std::thread;

mod camera;
mod iqm;

fn main() {
    use glium::DisplayBuild;
    use glium::Surface;

    // Create our display.
    let display = glium::glutin::WindowBuilder::new()
        .with_title("Rust IQM Viewer".to_string())
        .with_depth_buffer(24)
        .with_gl(glium::glutin::GlRequest::Latest)
        .build_glium()
        .unwrap();

    let mut f = File::open("data/mrfixit.iqm").unwrap();
    let mut file_buffer = Vec::new();

    f.read_to_end(&mut file_buffer).unwrap();

    let meshes = iqm::load_iqm(&display, file_buffer);

    // Setup Shaders
    let program = program!(&display, 140 => {
        vertex: include_str!("data/shader-140.vert"),
        fragment: include_str!("data/shader-140.frag"),
    }, 110 => {
        vertex: include_str!("data/shader-110.vert"),
        fragment: include_str!("data/shader-110.frag"),
    }).unwrap();

    // Create the camera.
    let mut camera = camera::Camera::new();

    // Prepare to start looping.
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    // The main loop
    loop {
        camera.update();

        let persp_matrix: [[f32; 4]; 4] = camera.get_perspective_matrix().into();
        let view_matrix: [[f32; 4]; 4] = camera.get_view_matrix().into();
        
        let uniforms = uniform! {
            persp_matrix: persp_matrix,
            view_matrix: view_matrix,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        for mesh in &meshes {
            // FIXME: the body is rendering properly, but the head isn't
            target.draw(&mesh.vertex_buffer, &mesh.index_buffer,
                        &program, &uniforms, &params).unwrap();
        }
        
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => process::exit(0),
                e => camera.process_input(&e),
            }
        }

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_timestamp = Duration::new(0, 16666667);

        // We don't really need this...
        while accumulator >= fixed_timestamp {
            accumulator -= fixed_timestamp;
        }

        thread::sleep(fixed_timestamp - accumulator);
    }
}
