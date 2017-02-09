#[macro_use]
extern crate glium;
extern crate byteorder;

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

    let meshes = iqm::load_iqm(&display, include_bytes!("data/mrfixit.iqm"));

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
        
        let uniforms = uniform! {
            persp_matrix: camera.get_perspective_matrix(),
            view_matrix: camera.get_view_matrix(),
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
            target.draw(&mesh.vertex_buffer,
                        &glium::index::NoIndices(glium::index::PrimitiveType::Points),
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
