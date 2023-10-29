
use std::{
    sync::mpsc::{
        self,
        Sender, Receiver,
    },
    thread::{self, JoinHandle},
    io::Write,
};

use crossterm::{
    self,
    terminal::{
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

pub struct DisplayModel {
    pub at_bat: AtBat,
}

pub struct AtBat {
    pub strikes: u8,
    pub balls: u8,
    pub outs: u8,

    pub base_state: [bool; 3],
}

pub struct Display<Out: Write + Send> {
    out: Out,
}

type Error = String;
type Result<T> = std::result::Result<T, Error>;

impl<Out: Write + Send + 'static> Display<Out> {
    pub fn start(writer: Out) -> (Sender<Option<DisplayModel>>, JoinHandle<()>) {
        let (sender, receiver) = mpsc::channel();
        let mut display = Self {
            out: writer,
        };

        let handle = thread::spawn(move || {
            if let Err(message) = display.run(receiver) {
                display.cleanup().expect(&format!("Failed to cleanup on error: {}", message));
                panic!("Display run: {}", message)
            }
        });

        (sender, handle)
    }

    fn run(&mut self, receiver: Receiver<Option<DisplayModel>>) -> Result<()> {
        self.init_terminal()?;

        while let Some(model) = receiver.recv().unwrap() {
            self.show(model)?;
        }

        self.cleanup()
    }

    fn init_terminal(&mut self) -> Result<()> {
        crossterm::execute!(self.out, EnterAlternateScreen)
            .map_err(|e| format!("{}", e))?;

        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        crossterm::execute!(self.out, LeaveAlternateScreen)
            .map_err(|e| format!("{}", e))?;

        Ok(())
    }

    fn show(&mut self, model: DisplayModel) -> Result<()> {
        todo!()
    }
}

