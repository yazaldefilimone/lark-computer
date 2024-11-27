#![allow(dead_code)]
use super::ram512::RAM512;
use crate::gates::{self, Bit16};

#[derive(Clone, Copy)]
pub struct RAM4K {
  ram512s: [RAM512; 8],
}

impl Default for RAM4K {
  fn default() -> Self {
    Self::new([RAM512::default(); 8])
  }
}

impl RAM4K {
  pub fn new(ram512s: [RAM512; 8]) -> Self {
    Self { ram512s }
  }

  pub fn tick(&mut self, address: [u8; 12], load: u8, input: Bit16) -> Bit16 {
    let (upper_addr, lower_addr) = (address[0..3].try_into().unwrap(), address[3..12].try_into().unwrap());

    let (load0, load1, load2, load3, load4, load5, load6, load7) = gates::dmux8way(load, upper_addr);

    self.ram512s[0].tick(lower_addr, load0, input);
    self.ram512s[1].tick(lower_addr, load1, input);
    self.ram512s[2].tick(lower_addr, load2, input);
    self.ram512s[3].tick(lower_addr, load3, input);
    self.ram512s[4].tick(lower_addr, load4, input);
    self.ram512s[5].tick(lower_addr, load5, input);
    self.ram512s[6].tick(lower_addr, load6, input);
    self.ram512s[7].tick(lower_addr, load7, input);

    gates::mux8way16(
      self.ram512s[0].tick(lower_addr, 0, input),
      self.ram512s[1].tick(lower_addr, 0, input),
      self.ram512s[2].tick(lower_addr, 0, input),
      self.ram512s[3].tick(lower_addr, 0, input),
      self.ram512s[4].tick(lower_addr, 0, input),
      self.ram512s[5].tick(lower_addr, 0, input),
      self.ram512s[6].tick(lower_addr, 0, input),
      self.ram512s[7].tick(lower_addr, 0, input),
      upper_addr,
    )
  }
}
