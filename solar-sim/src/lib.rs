use std::sync::Arc;

use log::warn;
use specs::{DispatcherBuilder, World, WorldExt};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event::*;
use winit::event_loop::EventLoop;
use winit::window::{CursorGrabMode, WindowBuilder};

use crate::control::Controls;
use crate::error::DynError;
use crate::render::camera::ControlCamera;
use crate::render::Render;
use crate::timer::Timer;

pub mod control;
pub mod error;
pub mod render;
pub mod texture;
pub mod timer;

pub async fn run() -> Result<(), DynError> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let window = Arc::new(window);
    if let Err(error) = window.set_cursor_grab(CursorGrabMode::Confined) {
        warn!("Failed to grab cursor: {error}")
    }
    window.set_cursor_visible(false);
    let state = Render::new(Arc::clone(&window)).await?;

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(Timer::default(), "timer", &[])
        .with(ControlCamera::default(), "camera", &["timer"])
        .with_thread_local(state)
        .build();
    dispatcher.setup(&mut world);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    #[cfg(not(target_arch = "wasm32"))]
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => control_flow.set_exit(),
                    WindowEvent::Resized(physical_size) => {
                        // TODO: state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &mut so w have to dereference it twice
                        // TODO: state.resize(**new_inner_size);
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        world.fetch_mut::<Controls>().process_keyboard(input);
                    }
                    _ => { /*TODO*/ }
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => world.fetch_mut::<Controls>().process_mouse(delta),
            Event::DeviceEvent {
                event: DeviceEvent::MouseWheel { delta },
                ..
            } => world.fetch_mut::<Controls>().process_wheel(delta),
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                dispatcher.dispatch(&world);
                world.maintain();
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn wasm_main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
    run().await.unwrap();
}
