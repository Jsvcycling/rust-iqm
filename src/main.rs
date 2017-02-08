#[macro_use]
extern crate glium;
extern crate byteorder;

use std::time::{Duration, Instant};
use std::process;
use std::thread;

use camera::Camera;
use iqm::load_iqm;

fn main() {
    use glium::DisplayBuild;

    // Create our display.
    let display = glium::glutin::WindowBuilder::new()
        .with_title("Rust IQM Viewer".to_string())
        .with_depth_buffer(24)
        .with_gl(glium::glutin::GlRequest::Latest)
        .build_glium()
        .unwrap();

    let vertex_buffer = load_iqm(&display, include_bytes!("data/teapot.iqm"));

    // Setup Shaders
    let program = program!(&display, 140 => {
        vertex: include_str!("data/shader-140.vert"),
        fragment: include_str!("data/shader-140.frag"),
    }, 120 => {
        vertex: include_str!("data/shader-120.vert"),
        fragment: include_str!("data/shader-120.frag"),
    });

    // Create the camera.
    let mut camera = Camera::new();

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
        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TriangleList),
                    &program, &uniforms, &params).unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => process::exit(0),
                e => camera.process_input(&e),
            }
        }

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_timestamp = Duration::new(0, 16666667);

        // We don't really need this...
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}
