use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::{
    dpi::{LogicalPosition, Position}, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}
};

pub struct WindowHandler {
    pub window: Option<Window>,
    pub window_id: Option<WindowId>,
    pub surface: Option<Surface>,
    pub device: Option<Device>,
    pub queue: Option<Queue>,
    pub config: Option<SurfaceConfiguration>
}

impl WindowHandler {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let window_attributes = Window::default_attributes()
            .with_title("migus engine")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 500.0))
            .with_position(Position::Logical(LogicalPosition::new(0.0, 0.0)));

        let window = event_loop.create_window(window_attributes).unwrap();
        let window_id = window.id();

        Self {
            window: Some(window),
            window_id: Some(window_id),
        }
    }

    pub fn handle_event(&self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(window) = &self.window {
            match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    window.request_redraw();
                }
                _ => (),
            }
        }
    }
}
