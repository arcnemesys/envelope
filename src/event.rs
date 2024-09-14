use crate::app::AppResult;
use ratatui::crossterm::event::{
    self, Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent,
};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread.
    handler: thread::JoinHandle<()>,
}
impl EventHandler {
    pub fn new() -> Self {
        EventHandler::default()
    }
        pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}
impl Default for EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || loop {
                if event::poll(Duration::from_millis(250)).expect("failed to poll new events") {
                    match event::read().expect("unable to read event") {
                        CrosstermEvent::Key(e) => {
                            if e.kind == KeyEventKind::Press {
                                sender.send(Event::Key(e))
                            } else {
                                Ok(())
                            }
                        }
                        CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                        CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                        CrosstermEvent::FocusGained => Ok(()),
                        CrosstermEvent::FocusLost => Ok(()),
                        CrosstermEvent::Paste(_) => unimplemented!(),
                    }
                    .expect("failed to send terminal event")
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }
}
