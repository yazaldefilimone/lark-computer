#![allow(dead_code)]

use crate::gates::{self, Bit16};

#[derive(Clone, Copy)]
pub struct DFF {
  state: u8,
}

impl Default for DFF {
  fn default() -> Self {
    Self::new(0)
  }
}

impl DFF {
  pub fn new(state: u8) -> Self {
    Self { state }
  }

  pub fn tick(&mut self, input: u8) {
    self.state = input;
  }

  pub fn output(&self) -> u8 {
    self.state
  }
}

// BIT
#[derive(Clone, Copy)]
pub struct Bit {
  dff: DFF,
}

impl Default for Bit {
  fn default() -> Self {
    Self::new(DFF::default())
  }
}

impl Bit {
  pub fn new(dff: DFF) -> Self {
    Self { dff }
  }

  pub fn tick(&mut self, input: u8, load: u8) {
    let next_state = gates::mux(self.dff.output(), input, load);
    self.dff.tick(next_state);
  }

  pub fn output(&self) -> u8 {
    self.dff.output()
  }
}

#[derive(Clone, Copy)]
pub struct Register {
  bits: [Bit; 16],
}

impl Register {
  pub fn new() -> Self {
    Self { bits: [Bit::default(); 16] }
  }

  pub fn tick(&mut self, input: Bit16, load: u8) {
    self.bits.iter_mut().enumerate().for_each(|(i, bit)| {
      bit.tick(input[i], load);
    });
  }

  pub fn output(&self) -> [u8; 16] {
    self.bits.map(|bit| bit.output())
  }
}
