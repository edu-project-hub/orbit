use std::{collections::HashMap, sync::Arc};

use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

#[derive(Debug)]
pub enum ShellError {
    EvLoop(EventLoopError),
}

pub struct Shell {
    windows: HashMap<WindowId, Arc<Window>>,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn run(mut self) -> Result<(), ShellError> {
        let evl = EventLoop::new().map_err(ShellError::EvLoop)?;
        evl.run_app(&mut self).map_err(ShellError::EvLoop)?;
        Ok(())
    }
}

impl ApplicationHandler for Shell {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            _ => {}
        }
    }
}
