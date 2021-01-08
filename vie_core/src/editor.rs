use crate::{
    backend::{Canvas, Event, EventLoop, Key},
    viewport::Viewport,
};
use anyhow::Result;
use thiserror::Error;

pub enum Transition {
    None,
}

pub trait Mode {
    fn recieve_input(&mut self, key: Key);

    fn next_transition(&self) -> Transition;
}

pub struct NormalMode {}
impl Mode for NormalMode {
    fn recieve_input(&mut self, key: Key) {}

    fn next_transition(&self) -> Transition {
        Transition::None
    }
}

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("there was an issue communicating with the underlying backend")]
    Io(#[from] std::io::Error),
    #[error("there was an issue drawing to the viewport")]
    Render(#[source] anyhow::Error),
}

/// The main application state.
pub struct Editor<'a, E: EventLoop, C: Canvas, M: Mode> {
    event_loop: E,
    viewport: Viewport<'a, C>,
    mode: M,
    should_quit: bool,
}

impl<'a, E: EventLoop, C: Canvas, M: Mode> Editor<'a, E, C, M> {
    pub fn run(&mut self) -> Result<(), EditorError> {
        while !self.should_quit {
            match self.event_loop.read_event()? {
                Event::Input(key) => self.mode.recieve_input(key),
                Event::Tick => (),
                Event::Error(e) => return Err(EditorError::from(e)),
            };

            match self.mode.next_transition() {
                Transition::None => (),
            };

            self.viewport
                .draw(|frame| Ok(()))
                .map_err(|e| EditorError::Render(e))?;
        }

        Ok(())
    }
}

impl<'a, E: EventLoop, C: Canvas> Editor<'a, E, C, NormalMode> {
    /// Create a new Editor.
    pub fn new(event_loop: E, canvas: &'a mut C) -> Result<Self> {
        use anyhow::Context;

        Ok(Self {
            event_loop,
            viewport: Viewport::new(canvas).context("unable to initialise Viewport")?,
            mode: NormalMode {},
            should_quit: false,
        })
    }
}
