#![allow(dead_code)]
use crate::gates::{Bit, Bit16};

type Bit2 = [Bit; 2];
type Bit3 = [Bit; 3];
// MULTIPLEXER GATE
//
// sel | ->
// ----+---
//  0  | a
//  1  | b
pub fn mux(a: Bit, b: Bit, sel: Bit) -> Bit {
  return if sel == 0 { a } else { b };
  // (a & !sel) | (b & sel)
}

// DEMULTIPLEXER GATE
//
// sel | a |  b
// ----+---+---
//  0  | n |  0
//  1  | 0 |  n
//
pub fn dmux(n: Bit, sel: Bit) -> (Bit, Bit) {
  if sel == 0 {
    return (n, 0);
  }
  return (0, n);
}

// 16-bit MUX
//
// a    = 1 | 0 | 1 | 0 | 1 | ....
// b    = 0 | 1 | 1 | 0 | 0 | ....
// sel  = 0 | 1 | 0 | 1 | 0 | ....
//       ---+---+---+---+---+
// ->   = 0 | 0 | 1 | 0 | 0 | ....
//
pub fn mux_16(a: Bit16, b: Bit16, sel: Bit) -> Bit16 {
  let mut out = [0; 16]; // TODO: use a slice
  for (pos, (bit_a, bit_b)) in a.into_iter().zip(b.into_iter()).enumerate() {
    out[pos] = mux(bit_a, bit_b, sel);
  }
  return out;
}

// 4-way MUX
//
// sel0 | sel1 | ->
// -----+------+---
//   0  |   0  |  a
//   1  |   0  |  b
//   0  |   1  |  c
//   1  |   1  |  d

pub fn mux_4_way_16(a: Bit16, b: Bit16, c: Bit16, d: Bit16, sel: Bit2) -> Bit16 {
  match sel {
    [0, 0] => a,
    [1, 0] => b,
    [0, 1] => c,
    [1, 1] => d,
    _ => panic!("error: sel value is not valid!"),
  }
}

pub fn mux_8_way_16(
  a: Bit16,
  b: Bit16,
  c: Bit16,
  d: Bit16,
  e: Bit16,
  f: Bit16,
  g: Bit16,
  h: Bit16,
  sel: Bit3,
) -> Bit16 {
  match sel {
    [0, 0, 0] => a,
    [0, 0, 1] => b,
    [0, 1, 0] => c,
    [0, 1, 1] => d,
    [1, 0, 0] => e,
    [1, 0, 1] => f,
    [1, 1, 0] => g,
    [1, 1, 1] => h,
    _ => panic!("error: sel value is not valid!"),
  }
}

pub fn dmux_4_way(n: Bit, sel: Bit2) -> (Bit, Bit, Bit, Bit) {
  match sel {
    [0, 0] => (n, 0, 0, 0),
    [1, 0] => (0, n, 0, 0),
    [0, 1] => (0, 0, n, 0),
    [1, 1] => (0, 0, 0, n),
    _ => panic!("error: sel value is not valid!"),
  }
}

pub fn dmux_8_way(n: Bit, sel: Bit3) -> (Bit, Bit, Bit, Bit, Bit, Bit, Bit, Bit) {
  match sel {
    [0, 0, 0] => (n, 0, 0, 0, 0, 0, 0, 0),
    [0, 0, 1] => (0, n, 0, 0, 0, 0, 0, 0),
    [0, 1, 0] => (0, 0, n, 0, 0, 0, 0, 0),
    [0, 1, 1] => (0, 0, 0, n, 0, 0, 0, 0),
    [1, 0, 0] => (0, 0, 0, 0, n, 0, 0, 0),
    [1, 0, 1] => (0, 0, 0, 0, 0, n, 0, 0),
    [1, 1, 0] => (0, 0, 0, 0, 0, 0, n, 0),
    [1, 1, 1] => (0, 0, 0, 0, 0, 0, 0, n),
    _ => panic!("error: sel value is not valid!"),
  }
}
