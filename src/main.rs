pub mod ecs;
pub mod renderer;
pub mod transform;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    pollster::block_on(run());
}

pub fn run_systems(renderer: &mut renderer::Renderer, world: &mut ecs::World) {
    renderer.run(world);
    world.clear_events();
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut renderer = renderer::Renderer::new(&window).await;

    let mut world = ecs::World::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                world.window_events.push(ecs::WindowEvent::Resized(
                    physical_size.width,
                    physical_size.height,
                ));
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                world.window_events.push(ecs::WindowEvent::Resized(
                    (*new_inner_size).width,
                    (*new_inner_size).height,
                ));
                // new_inner_size is &&mut so we have to dereference it twice
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            run_systems(&mut renderer, &mut world);
            if world.renderer_outofmemory_error {
                *control_flow = ControlFlow::Exit
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {}
    });
}
