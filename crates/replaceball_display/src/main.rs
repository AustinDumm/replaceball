use std::{
    io::Write,
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode};
use display::{DisplayModel, AtBat};

mod display;

fn main() -> std::io::Result<()> {
    let mut stderr = std::io::stderr();
    let (sender, join_handle) = display::Display::start(std::io::stdout());

    loop {
        match poll_esc() {
            Ok(true) => break,
            Ok(false) => (),
            Err(e) => {
                write!(stderr, "Error polling: {}", e)?;
                break;
            },
        }

        if let Err(e) = sender.send(Some(DisplayModel {
            at_bat: AtBat {
                strikes: 0,
                balls: 0,
                outs: 0,
                base_state: [false, false, false],
            },
        })) {
            write!(stderr, "Failed to send model: {}", e)?;
            break;
        }
    }

    let close_send = sender.send(None);
    let display_result = join_handle.join().expect("Failed to join to display thread");

    match (close_send, display_result) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(e), Ok(_)) => panic!("Failed to send closing message to display thread: {}", e),
        (_, Err(e)) => Err(e),
    }
}

fn poll_esc() -> Result<bool, String> {
    if event::poll(Duration::from_secs(1))
        .map_err(|e| format!("{}", e))? {
        let event = event::read()
            .map_err(|e| format!("{}", e))?;

        if event == Event::Key(KeyCode::Esc.into()) {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}
