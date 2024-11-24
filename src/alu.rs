#![allow(dead_code)]

use crate::gates::{self, Bit, Bit16};

/*
+ ---------------------------------- +
| zx | nx | zy | ny | f | no |  out  |
+ ---------------------------------- +
|  1 |  0 |  1 |  0 |  1 |  0 |   0  |
|  1 |  1 |  1 |  1 |  1 |  1 |   1  |
|  1 |  1 |  1 |  0 |  1 |  0 |  -1  |
|  0 |  0 |  1 |  1 |  0 |  0 |   x  |
|  1 |  1 |  0 |  0 |  0 |  0 |   y  |
|  0 |  0 |  1 |  1 |  0 |  1 |  !x  |
|  1 |  1 |  0 |  0 |  0 |  1 |  !y  |
|  0 |  0 |  1 |  1 |  1 |  1 |  -x  |
|  1 |  1 |  0 |  0 |  1 |  1 |  -y  |
|  0 |  1 |  1 |  1 |  1 |  1 |  x+1 |
|  1 |  1 |  0 |  1 |  1 |  1 |  y+1 |
|  0 |  0 |  1 |  1 |  1 |  0 |  x-1 |
|  1 |  1 |  0 |  0 |  1 |  0 |  y-1 |
|  0 |  0 |  0 |  0 |  1 |  0 |  x+y |
|  0 |  1 |  0 |  0 |  1 |  1 |  x-y |
|  0 |  0 |  0 |  1 |  1 |  1 |  y-x |
|  0 |  0 |  0 |  0 |  0 |  0 |  x&y |
|  0 |  1 |  0 |  1 |  0 |  1 |  x|y |
+ ---------------------------------- +
Key:
  zx: Zero the X input
  nx: Negate the X input
  zy: Zero the Y input
  ny: Negate the Y input
  f:  Select between ADD (x+y) or AND (x&y)
  no: Negate the Output

  zr: if out == 0 then zr = 1 else zr = 0
  ng: if out < 0 then ng = 1 else ng = 0

papers:
  - [Introduction to “The First Draft Report on the EDVAC” by John von Neumann]
  (https://people.csail.mit.edu/brooks/idocs/VonNeumann_EDVAC.pdf)
*/

pub struct ALU {
  // input
  x: Bit16,
  y: Bit16,
  // output
  out: Bit16,
  zr: Bit, // if out == 0, zr = 1 else zr = 0 (zero flag)
  ng: Bit, // if out < 0, ng = 1 else ng = 0 (negative flag)

  // control bits
  zx: Bit, // if zx ==1 than x = 0
  nx: Bit, // if nx ==1 than x = !x
  zy: Bit, // zero the y input
  ny: Bit, // negate the y input
  f: Bit,  // f == 1, out=add(x,y), else out=and(x,y)
  no: Bit, // negate the out
}

impl Default for ALU {
  fn default() -> Self {
    let x = [0; 16];
    let y = [0; 16];
    Self::new(x, y)
  }
}

impl ALU {
  pub fn new(x: Bit16, y: Bit16) -> Self {
    Self { x, y, out: [0; 16], zr: 0, ng: 0, zx: 0, nx: 0, zy: 0, ny: 0, f: 0, no: 0 }
  }

  // setters control bits
  pub fn set_zx(&mut self, bit: Bit) {
    self.zx = bit;
  }
  pub fn set_nx(&mut self, bit: Bit) {
    self.nx = bit;
  }
  pub fn set_zy(&mut self, bit: Bit) {
    self.zy = bit;
  }
  pub fn set_ny(&mut self, bit: Bit) {
    self.ny = bit;
  }
  pub fn set_f(&mut self, bit: Bit) {
    self.f = bit;
  }

  pub fn set_no(&mut self, bit: Bit) {
    self.no = bit;
  }

  pub fn set_controls(&mut self, bits: [Bit; 6]) {
    self.set_zx(bits[0]);
    self.set_nx(bits[1]);
    self.set_zy(bits[2]);
    self.set_ny(bits[3]);
    self.set_f(bits[4]);
    self.set_no(bits[5]);
  }

  // operators
  fn zero_x(&mut self) {
    if self.zx == 1 {
      // set as 00000000000000000
      self.x = [0; 16];
    }
  }

  fn negate_x(&mut self) {
    if self.nx == 1 {
      // bit-wise not
      self.x = gates::not_16(self.x);
    }
  }

  fn zero_y(&mut self) {
    if self.zy == 1 {
      // set as 00000000000000000
      self.y = [0; 16];
    }
  }

  fn negate_y(&mut self) {
    if self.ny == 1 {
      // bit-wise not
      self.y = gates::not_16(self.y);
    }
  }

  fn fn_out(&mut self) {
    if self.f == 1 {
      self.out = gates::add_16(self.x, self.y);
    } else {
      self.out = gates::and_16(self.x, self.y);
    }
  }

  fn negate_out(&mut self) {
    if self.no == 1 {
      self.out = gates::not_16(self.out)
    }
  }

  // output control
  fn zero_flag(&mut self) {
    self.zr = if self.out == [0; 16] { 1 } else { 0 };
  }

  fn negate_flag(&mut self) {
    self.ng = if self.out < [0; 16] { 1 } else { 0 };
  }

  pub fn exc(&mut self) -> (Bit16, Bit, Bit) {
    // 1.
    self.zero_x();
    self.negate_x();
    // 2.
    self.zero_y();
    self.negate_y();
    // 3.
    self.fn_out();
    // 4.
    self.negate_out();
    // 5.
    self.zero_flag();
    self.negate_flag();
    return (self.out, self.zr, self.ng);
  }

  // debug
  pub fn show(&self) {
    println!("+ ---------------------------------- +");
    println!("| zx | nx | zy | ny | f | no |  out  |");
    println!("+ ---------------------------------- +");
    println!(
      "|  {} |  {} |  {} |  {} |  {} |  {} |  {:?}  |",
      self.zx, self.nx, self.zy, self.ny, self.f, self.no, self.out
    );
    println!("+ ---------------------------------- +");
  }
}
