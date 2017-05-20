extern crate cgmath;
extern crate glutin;
extern crate three;

use cgmath::prelude::*;

fn main() {
    let builder = glutin::WindowBuilder::new()
                                        .with_title("Three-rs line drawing example");
    let event_loop = glutin::EventsLoop::new();
    let (mut renderer, mut factory) = three::Renderer::new(builder, &event_loop);

    let mut camera = three::PerspectiveCamera::new(45.0, renderer.get_aspect(), 1.0, 500.0);
    camera.position = three::Position::new(0.0, 0.0, 100.0);
    camera.look_at(three::Position::new(0.0, 0.0, 0.0));

    let geometry = three::Geometry::from_vertices(vec![
        three::Position::new(-10.0, 0.0, 0.0),
        three::Position::new(0.0, 10.0, 0.0),
        three::Position::new(10.0, 0.0, 0.0),
    ]);
    let material = three::Material::LineBasic { color: 0x0000ff };
    let mut line = factory.line(geometry, material);

    let mut scene = factory.scene();
    scene.add(&mut line, None);

    let mut angle = 0f32;
    let mut running = true;
    while running {
        event_loop.poll_events(|glutin::Event::WindowEvent {event, ..}| {
            use glutin::WindowEvent as Event;
            use glutin::VirtualKeyCode as Key;
            match event {
                Event::Resized(..) => {
                    renderer.resize();
                    camera.projection.aspect = renderer.get_aspect();
                }
                Event::KeyboardInput(_, _, Some(Key::Escape), _) |
                Event::Closed => {
                    running = false
                }
                _ => ()
            }
        });

        angle += 0.01f32;
        line.transform_mut().rot = three::Orientation::from_axis_angle(
            cgmath::Vector3::unit_z(), cgmath::Rad(angle));

        scene.update();
        renderer.render(&scene, &camera);
    }
}
