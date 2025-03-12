mod window;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};
use window::WindowHandler;

#[derive(Default)]
pub struct App {
    window_handler: Option<WindowHandler>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window_handler = Some(WindowHandler::new(event_loop));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if let Some(handler) = &self.window_handler {
            handler.handle_event(event_loop, id, event);
        }
    }
}

impl App {
    pub fn init() {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);

        let mut app = App::default();
        let _ = event_loop.run_app(&mut app);
    }
}
