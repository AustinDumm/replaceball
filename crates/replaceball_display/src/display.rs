
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
        self,
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

type Error = std::io::Error;
type Result<T> = std::result::Result<T, Error>;

impl<Out: Write + Send + 'static> Display<Out> {
    pub fn start(writer: Out) -> (Sender<Option<DisplayModel>>, JoinHandle<Result<()>>) {
        let (sender, receiver) = mpsc::channel();
        let mut display = Self {
            out: writer,
        };

        let handle = thread::spawn(move || {
            let result = display.run(receiver);
            display.cleanup()?;

            result
        });

        (sender, handle)
    }

    fn run(&mut self, receiver: Receiver<Option<DisplayModel>>) -> Result<()> {
        self.init_terminal()?;

        while let Some(model) = receiver.recv().unwrap() {
            if let Err(e) = self.show(model) {
                self.cleanup().expect("Failed to cleanup");
                return Err(e)
            }
        }

        self.cleanup()
    }

    fn init_terminal(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(
            self.out,
            EnterAlternateScreen
        )
    }

    fn cleanup(&mut self) -> Result<()> {
        crossterm::execute!(
            self.out,
            LeaveAlternateScreen
        )?;
        terminal::disable_raw_mode()
    }

    fn show(&mut self, model: DisplayModel) -> Result<()> {
        Err(Error::new(std::io::ErrorKind::Other, "Not yet implemented"))
    }
}

