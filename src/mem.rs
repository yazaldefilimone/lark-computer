use crate::gates::{self, Bit16, Bit8};

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

impl Bit {
  pub fn new() -> Self {
    Self { dff: DFF::default() }
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
    Self { bits: [Bit::new(); 16] }
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

pub struct RAM64 {
  ram8s: [RAM8; 8],
}

impl Default for RAM64 {
  fn default() -> Self {
    Self::new([RAM8::default(); 8])
  }
}

impl RAM64 {
  pub fn new(ram8s: [RAM8; 8]) -> Self {
    Self { ram8s }
  }

  pub fn tick(&mut self, address: [u8; 6], load: u8, input: Bit16) -> Bit16 {
    let (upper_addr, lower_addr) = (address[0..3].try_into().unwrap(), address[3..6].try_into().unwrap());

    let (load0, load1, load2, load3, load4, load5, load6, load7) = gates::dmux8way(load, upper_addr);

    self.ram8s[0].tick(lower_addr, load0, input);
    self.ram8s[1].tick(lower_addr, load1, input);
    self.ram8s[2].tick(lower_addr, load2, input);
    self.ram8s[3].tick(lower_addr, load3, input);
    self.ram8s[4].tick(lower_addr, load4, input);
    self.ram8s[5].tick(lower_addr, load5, input);
    self.ram8s[6].tick(lower_addr, load6, input);
    self.ram8s[7].tick(lower_addr, load7, input);

    gates::mux8way16(
      self.ram8s[0].tick(lower_addr, 0, input),
      self.ram8s[1].tick(lower_addr, 0, input),
      self.ram8s[2].tick(lower_addr, 0, input),
      self.ram8s[3].tick(lower_addr, 0, input),
      self.ram8s[4].tick(lower_addr, 0, input),
      self.ram8s[5].tick(lower_addr, 0, input),
      self.ram8s[6].tick(lower_addr, 0, input),
      self.ram8s[7].tick(lower_addr, 0, input),
      upper_addr,
    )
  }
}
