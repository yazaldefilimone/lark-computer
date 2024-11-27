#![allow(dead_code)]
use super::ram4k::RAM4K;
use crate::gates::{self, Bit16};

#[derive(Clone, Copy)]
pub struct RAM16K {
  ram4ks: [RAM4K; 4],
}

impl Default for RAM16K {
  fn default() -> Self {
    Self::new([RAM4K::default(); 4])
  }
}

impl RAM16K {
  pub fn new(ram4ks: [RAM4K; 4]) -> Self {
    Self { ram4ks }
  }

  pub fn tick(&mut self, address: [u8; 14], load: u8, input: Bit16) -> Bit16 {
    let (upper_addr, lower_addr) = (address[0..2].try_into().unwrap(), address[2..14].try_into().unwrap());

    let (load0, load1, load2, load3) = gates::dmux4way(load, upper_addr);

    self.ram4ks[0].tick(lower_addr, load0, input);
    self.ram4ks[1].tick(lower_addr, load1, input);
    self.ram4ks[2].tick(lower_addr, load2, input);
    self.ram4ks[3].tick(lower_addr, load3, input);

    gates::mux4way16(
      self.ram4ks[0].tick(lower_addr, 0, input),
      self.ram4ks[1].tick(lower_addr, 0, input),
      self.ram4ks[2].tick(lower_addr, 0, input),
      self.ram4ks[3].tick(lower_addr, 0, input),
      upper_addr,
    )
  }
}
