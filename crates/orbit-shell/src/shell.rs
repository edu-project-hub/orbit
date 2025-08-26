use std::{collections::HashMap, sync::Arc};

use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Window, WindowId},
};

use crate::event::Event;

#[derive(Debug)]
pub enum ShellError {
    EvLoop(EventLoopError),
}

pub struct Shell {
    windows: HashMap<WindowId, Arc<Window>>,
    evl: EventLoop<Event>,
    proxy: EventLoopProxy<Event>,
}

impl Shell {
    pub fn new() -> Result<Self, ShellError> {
        let evl = EventLoop::with_user_event()
            .build()
            .map_err(ShellError::EvLoop)?;
        let proxy = evl.create_proxy();

        Ok(Self {
            windows: HashMap::new(),
            evl,
            proxy,
        })
    }

    pub fn run(mut self) -> Result<(), ShellError> {
        /*  let evl = EventLoop::new().map_err(ShellError::EvLoop)?;
        evl.run_app(&mut self).map_err(ShellError::EvLoop)?;
        Ok(()) */

        Ok(())
    }
}

impl ApplicationHandler<Event> for Shell {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

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
