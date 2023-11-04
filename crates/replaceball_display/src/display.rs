
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
        LeaveAlternateScreen, BeginSynchronizedUpdate, EndSynchronizedUpdate, Clear, ClearType, WindowSize,
    },
    cursor::*,
    style::*,
};

use crate::ring_buffer::RingBuffer;

#[derive(Clone)]
pub struct DisplayModel {
    pub is_top: bool,
    pub inning_index: u8,
    pub score: Score,
    pub at_bat: AtBat,
    pub event_record: EventRecord,
}

#[derive(Clone)]
pub struct Score {
    pub home: u16,
    pub away: u16,
}

#[derive(Clone)]
pub struct AtBat {
    pub strikes: u8,
    pub balls: u8,
    pub outs: u8,

    pub base_state: [bool; 3],
}

#[derive(Debug, Clone)]
pub struct EventRecord {
    pub event_list: RingBuffer<String>,
}

pub struct Display<Out: Write + Send> {
    out: Out,
}

struct Rect {
    width: u16,
    height: u16,
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
            EnterAlternateScreen,
            Hide,
        )
    }

    fn cleanup(&mut self) -> Result<()> {
        crossterm::execute!(
            self.out,
            LeaveAlternateScreen,
            Show,
        )?;
        terminal::disable_raw_mode()
    }

    fn show(&mut self, model: DisplayModel) -> Result<()> {
        crossterm::queue!(
            self.out,
            BeginSynchronizedUpdate,
            Clear(ClearType::All),
            MoveTo(0, 0),
            SavePosition,
        )?;

        let score_size = self.enqueue_score(&model)?;

        crossterm::queue!(
            self.out,
            RestorePosition,
            MoveDown(score_size.height),
            EndSynchronizedUpdate,
            SavePosition,
        )?;

        let at_bat_size = self.enqueue_at_bat(&model.at_bat)?;

        crossterm::queue!(
            self.out,
            RestorePosition,
            MoveDown(at_bat_size.height),
            SavePosition,
        )?;

        _ = self.enqueue_events(&model)?;

        self.out.flush()?;
        Ok(())
    }

    fn enqueue_at_bat(&mut self, model: &AtBat) -> Result<Rect> {
        crossterm::queue!(
            self.out,
            Print(format!(
                " {}-{}  {} Out(s)  {}{}{}⌂",
                model.balls,
                model.strikes,
                model.outs,
                if model.base_state[0] { "⬥" } else { "⬦" },
                if model.base_state[1] { "⬥" } else { "⬦" },
                if model.base_state[2] { "⬥" } else { "⬦" },
            )),
        )?;

        Ok(Rect { width: 0, height: 2 })
    }

    fn enqueue_score(&mut self, model: &DisplayModel) -> Result<Rect> {
        let score_string = format!(
            " Away {:<2} -{}{}- {:2} Home ",
            model.score.away,
            if model.is_top { "▲" } else { "▼" },
            model.inning_index + 1,
            model.score.home
        );
        let score_width = score_string.chars().count();
        let box_horizontal = format!("+{}+", (0..score_width).map(|_| "=").collect::<String>());

        crossterm::queue!(
            self.out,
            SavePosition,
            Print(format!(
                "{}",
                box_horizontal
            )),
            RestorePosition,
            SavePosition,
            MoveDown(1),
            Print(format!(
                "|{}|",
                score_string,
            )),
            RestorePosition,
            MoveDown(2),
            Print(format!(
                "{}",
                box_horizontal,
            )),
        )?;

        Ok(Rect { width: score_width as u16, height: 3 })
    }
    
    fn enqueue_events(&mut self, model: &DisplayModel) -> Result<Rect> {
        let WindowSize {
            rows: _,
            columns,
            width: _,
            height: _,
        } = terminal::window_size()?;

        // Gives margin
        let columns = (columns - 10) as usize;
        let mut total_rows = 0;
        for event in model.event_record.event_list.iter().rev() {
            let mut event_chars = event.chars();
            let char_count = event_chars.clone().count();
            let row_count = (char_count / columns) + 1;
            total_rows += row_count;

            for row in 0..row_count {
                crossterm::queue!(
                    self.out,
                    SavePosition,
                    MoveLeft(if row == 0 { 0 } else { 4 }),
                    Print(event_chars
                              .by_ref()
                              .take(columns)
                              .collect::<String>()
                    ),
                    RestorePosition,
                    MoveDown(1),
                )?;
            }
        }

        Ok(Rect {
            width: 0,
            height: total_rows as u16,
        })
    }
}

