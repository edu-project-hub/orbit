use winit::{error::EventLoopError, event_loop::EventLoop};

use crate::{app::App, event::AppEvent};

pub mod app;
pub mod event;

#[derive(Debug)]
pub enum EventError {
    Event(EventLoopError),
}

pub fn runner() -> Result<(), EventError> {
    let evl = EventLoop::<AppEvent>::with_user_event()
        .build()
        .map_err(EventError::Event)?;

    let proxy = evl.create_proxy();
    let mut app = App::new(proxy);
    evl.run_app(&mut app).map_err(EventError::Event)?;
    Ok(())
}
