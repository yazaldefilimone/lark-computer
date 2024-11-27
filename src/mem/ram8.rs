#![allow(dead_code)]
use crate::gates::{self, Bit16};

use super::register::Register;

#[derive(Clone, Copy)]
pub struct RAM8 {
  regs: [Register; 8],
}

impl Default for RAM8 {
  fn default() -> Self {
    Self::new([Register::new(); 8])
  }
}

impl RAM8 {
  pub fn new(regs: [Register; 8]) -> Self {
    Self { regs }
  }

  pub fn tick(&mut self, address: [u8; 3], load: u8, input: Bit16) -> Bit16 {
    let (load0, load1, load2, load3, load4, load5, load6, load7) = gates::dmux8way(load, address);
    self.regs[0].tick(input, load0);
    self.regs[1].tick(input, load1);
    self.regs[2].tick(input, load2);
    self.regs[3].tick(input, load3);
    self.regs[4].tick(input, load4);
    self.regs[5].tick(input, load5);
    self.regs[6].tick(input, load6);
    self.regs[7].tick(input, load7);

    gates::mux8way16(
      self.regs[0].output(),
      self.regs[1].output(),
      self.regs[2].output(),
      self.regs[3].output(),
      self.regs[4].output(),
      self.regs[5].output(),
      self.regs[6].output(),
      self.regs[7].output(),
      address,
    )
  }
}
