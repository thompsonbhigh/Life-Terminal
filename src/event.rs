use std::{io, time::Duration};

use crossterm::event::{self, Event};

pub fn next(timeout: Duration) -> io::Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}
