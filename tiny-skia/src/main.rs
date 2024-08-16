use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
fn main() {
    let event_loop = EventLoop::new().unwrap();

    // TODO use ActiveEventLoop
    let window = event_loop
        .create_window(Window::default_attributes())
        .unwrap();

    // TODO use EventLoop::run_app
    event_loop.set_control_flow(ControlFlow::Wait);
    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => elwt.exit(),
        Event::AboutToWait => window.request_redraw(),
        Event::WindowEvent {
            window_id,
            event: WindowEvent::RedrawRequested,
        } if window_id == window.id() => {}
        _ => (),
    });
}
