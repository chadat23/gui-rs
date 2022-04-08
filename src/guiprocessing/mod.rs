use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::guiproperties::guiposition::GUISize;
// use crate::guiproperties::guitraits::{Widget, Parent};
// use crate::guiresources::GUIResources;
// use crate::guiwidgets::GUIWindow;
use crate::guiresources::GUIResources;
use crate::guiwidgets::GUIBase;

// mod state;
pub mod processing_utils;
pub mod vertices;

mod state;
use state::State;

/// The main funciton that executes everthing.
pub fn run(mut guibase: GUIBase, guiresources: GUIResources) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    guibase.logical_scale = Some(window.scale_factor());
    // guiwindow.set_id(0);
    // (guiwindow.children, _) = window_building_utils::set_widget_ids(guiwindow.give_children(), 1);
    let guibase = guibase;
    let window = processing_utils::set_window_properties(
        window,
        &guibase,
        guibase.windows.get(&-1).unwrap().get_window(),
    );

    // State::new uses async code, so we're going to wait for it to finish
    let mut state: State = pollster::block_on(State::new(&window, guibase, guiresources));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
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
                            state.resize(GUISize::from_physical_pixels(
                                physical_size.width as f64,
                                physical_size.height as f64,
                                &state.guibase.logical_scale.unwrap(),
                            ));
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            state.resize(GUISize::from_physical_pixels(
                                new_inner_size.width as f64,
                                new_inner_size.height as f64,
                                &state.guibase.logical_scale.unwrap(),
                            ));
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // state.update();
                match state.render() {
                    Ok(_) => {
                        // println!("good!");
                        // println!("{:?}", SystemTime::now());
                    }
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => {
                        state.size;
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}