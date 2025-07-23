mod vulkan_app;
mod camera;

use vulkan_app::{VulkanApp, HEIGHT, WIDTH};
use camera::{Camera, CameraMovement};
use winit::event::{
    DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent,
};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, WindowBuilder};
use winit::dpi::PhysicalPosition;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vulkan Triangle")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut app = VulkanApp::new(&window);
    let mut camera = Camera::new(cgmath::Vector3::new(2.0, 2.0, 2.0), -135.0, -35.0);

    let mut input_state = InputState::default();
    let mut last_frame = std::time::Instant::now();
    let mut focused = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => {
                    if new_size.width > 0 && new_size.height > 0 {
                        app.framebuffer_resized = true;
                    }
                }
                WindowEvent::Focused(f) => {
                    focused = f;
                    if focused {
                        let _ = window.set_cursor_grab(CursorGrabMode::Locked);
                        window.set_cursor_visible(false);
                        let size = window.inner_size();
                        let _ = window.set_cursor_position(PhysicalPosition::new(
                            size.width as f64 / 2.0,
                            size.height as f64 / 2.0,
                        ));
                    } else {
                        let _ = window.set_cursor_grab(CursorGrabMode::None);
                        window.set_cursor_visible(true);
                        input_state = InputState::default();
                    }
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if focused {
                        if let Some(key) = input.virtual_keycode {
                            let pressed = input.state == ElementState::Pressed;
                            match key {
                                VirtualKeyCode::W => input_state.forward = pressed,
                                VirtualKeyCode::S => input_state.backward = pressed,
                                VirtualKeyCode::A => input_state.left = pressed,
                                VirtualKeyCode::D => input_state.right = pressed,
                                VirtualKeyCode::Space => input_state.up = pressed,
                                VirtualKeyCode::LShift => input_state.down = pressed,
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            },
            Event::DeviceEvent { event, .. } => {
                if focused {
                    if let DeviceEvent::MouseMotion { delta } = event {
                        camera.process_mouse(delta.0 as f32, delta.1 as f32);
                    }
                }
            }
            Event::MainEventsCleared => {
                let now = std::time::Instant::now();
                let dt = now.duration_since(last_frame).as_secs_f32();
                last_frame = now;

                if focused {
                    if input_state.forward { camera.process_keyboard(CameraMovement::Forward, dt); }
                    if input_state.backward { camera.process_keyboard(CameraMovement::Backward, dt); }
                    if input_state.left { camera.process_keyboard(CameraMovement::Left, dt); }
                    if input_state.right { camera.process_keyboard(CameraMovement::Right, dt); }
                    if input_state.up { camera.process_keyboard(CameraMovement::Up, dt); }
                    if input_state.down { camera.process_keyboard(CameraMovement::Down, dt); }
                }

                app.draw_frame(&window, &camera);
            }
            _ => {}
        }
    });
}

#[derive(Default)]
struct InputState {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}
