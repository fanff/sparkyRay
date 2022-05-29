use winit::dpi::{LogicalPosition, PhysicalPosition};
use winit::event::ElementState::Pressed;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent::KeyboardInput;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("render_live");
    window.set_cursor_visible(false);
    //window.set_cursor_grab(true).expect("TODO: panic message");

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        // *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                println!("CursorMoved {:?} ", position);
            }

            Event::WindowEvent {
                event: WindowEvent::AxisMotion { value, axis, .. },
                ..
            } => {
                //println!("AxisMotion {:?}  {:?}", value, axis);

                //window.set_cursor_grab(true).expect("TODO: panic message");
            }

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => match (input.state, input.scancode, input.virtual_keycode.unwrap()) {
                (Pressed, _scan_code, VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,

                (Pressed, _scan_code, VirtualKeyCode::Z) => {}
                (Pressed, _scan_code, VirtualKeyCode::S) => {}

                _ => {
                    println!("{:?}", input);
                }
            },

            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                // println!("yoyoyoy");
                //window.set_cursor_position(PhysicalPosition::new(100.0, 100.0));
                //window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                //let rwh = window();

                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // println!("yoyoyoy");
            }
            _ => (),
        }
    });
}
