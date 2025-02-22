use std::time::{Duration, Instant};
use std::io;
use crossterm::event::{self, Event};
use ratatui::{backend::Backend, Terminal};
use rand::Rng;
use crate::{ui, input::TextArea};

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut data: Vec<u64> = (0..100).map(|_| rand::rng().random_range(0..50)).collect();
    let mut rng = rand::rng();
    let mut text_area = TextArea::new();
    let mut last_update = Instant::now();

    loop {
        if last_update.elapsed() > Duration::from_millis(200) {
            data.remove(0);
            data.push(rng.random_range(0..50));
            last_update = Instant::now();
        }

        terminal.draw(|f| ui::draw(f, &data, &text_area.get_text()))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if text_area.handle_key_event(key) {
                    break;
                }
            }
        }
    }
    Ok(())
}

