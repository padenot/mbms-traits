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

pub enum BelaPort {
  // Audio input, 0 or 1
  AudioIn(usize),
  // Audio output, 0 or 1
  AudioOut(usize),
  // Digital IO, 0 to 15
  Digital(usize),
  // Analog input, 0 to 7
  AnalogIn(usize),
  // Analog output, 0 to 7
  AnalogOut(usize),
}
