use std::{sync::Arc, thread::JoinHandle};

use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopClosed, EventLoopProxy},
    window::WindowId,
};

use crate::event::Event;

#[derive(Debug)]
pub enum ShellError<T> {
    EventLoopError(EventLoopError),
    EventLoopClosed(EventLoopClosed<T>),
}

pub struct Shell<T>
where
    T: 'static,
{
    proxy: EventLoopProxy<T>,§  
}

impl<T> ApplicationHandler<T> for Shell<T>
where
    T: 'static,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        todo!()
    }
}

pub fn runner() -> Result<(), ShellError<Event>> {
    let evl: EventLoop<Event> = EventLoop::with_user_event()
        .build()
        .map_err(ShellError::EventLoopError)?;

    let proxy = evl.create_proxy();
    let mut shell = Shell { proxy };

    evl.run_app(&mut shell)
        .map_err(ShellError::EventLoopError)?;

    Ok(())
}
