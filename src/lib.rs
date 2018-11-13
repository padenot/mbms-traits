extern crate monome;
extern crate bela;

use monome::MonomeEvent;
use bela::Context;

pub trait InstrumentRenderer {
    fn render(&mut self, context: &mut Context);
}

pub trait InstrumentControl {
    fn input(&mut self, event: MonomeEvent);
    fn main_thread_work(&mut self);
    fn render(&mut self, grid: &mut [u8; 128]);
}
