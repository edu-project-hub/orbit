use std::collections::HashMap;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoopProxy},
    window::{Window, WindowAttributes, WindowId},
};

use crate::event::AppEvent;

pub struct App {
    proxy: EventLoopProxy<AppEvent>,
    windows: HashMap<WindowId, Window>,
}

impl App {
    pub fn new(proxy: EventLoopProxy<AppEvent>) -> Self {
        App { proxy, windows: HashMap::new() }
    }
}

impl ApplicationHandler<AppEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = WindowAttributes::default().with_title("orbit");
        let window = event_loop.create_window(attrs).unwrap();
        self.windows.insert(window.id(), window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.windows.remove(&window_id);
                if self.windows.is_empty() {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}
