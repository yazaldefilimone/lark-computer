#![allow(dead_code)]

pub type Bit = u8;
pub type Bit2 = [Bit; 2];
pub type Bit16 = [Bit; 16];

// NAND GATE (BASE)
//
// a  | b  | ->
// ---+----+----
//  0 |  0 |  1
//  0 |  1 |  1
//  1 |  0 |  1
//  1 |  1 |  0
pub fn nand(a: Bit, b: Bit) -> Bit {
  !(a & b) & 1
}

// NOT GATE
//
// a  | ->
// ---+----
//  0 |  1
//  1 |  0
pub fn not(a: Bit) -> Bit {
  nand(a, 1)
}

// AND GATE
// a  | b  | ->
// ---+----+----
//  0 |  0 |  0
//  0 |  1 |  0
//  1 |  0 |  0
//  1 |  1 |  1
pub fn and(a: Bit, b: Bit) -> Bit {
  nand(nand(a, b), nand(a, b))
}

// OR GATE
//
// a  | b  | ->
// ---+----+----
//  0 |  0 |  0
//  0 |  1 |  1
//  1 |  0 |  1
//  1 |  1 |  1
pub fn or(a: Bit, b: Bit) -> Bit {
  nand(not(a), not(b))
}

// XOR GATE
//
// a  | b  | ->
// ---+----+----
//  0 |  0 |  0
//  0 |  1 |  1
//  1 |  0 |  1
//  1 |  1 |  0
pub fn xor(a: Bit, b: Bit) -> Bit {
  let not_a_and_b = nand(nand(a, 1), b);
  let a_and_not_b = nand(a, nand(b, 1));
  nand(not_a_and_b, a_and_not_b)
  // nand(nand(a, nand(a, b)), nand(b, nand(a, b)))
}

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
// a  | b  | sel | ->
// ---+----+-----+----
//  0 |  0 |  0  |  0
//  0 |  0 |  1  |  1
//  0 |  1 |  0  |  1
//  0 |  1 |  1  |  0
//  1 |  0 |  0  |  1
//  1 |  0 |  1  |  0
//  1 |  1 |  0  |  0
//  1 |  1 |  1  |  0
pub fn dmux(x: Bit, sel: Bit) -> (Bit, Bit) {
  let a = x & !sel;
  let b = x & sel;
  (a, b)
}

// 16-bit gates (not, and, or, mux)
// todo: using simd to implement these gates?
//

// 16-bit NOT
// a  = 1 | 0 | 1 | 0 | 1 | ....
// -> = 0 | 1 | 0 | 1 | 0 | ....
//
pub fn not_16(a: Bit16) -> Bit16 {
  return a.map(|x| not(x));
}

// 16-bit AND
// a  = 1 | 0 | 1 | 0 | 1 | ....
// b  = 0 | 1 | 1 | 0 | 0 | ....
//     ---+---+---+---+---+
// -> = 0 | 0 | 1 | 0 | 0 | ....
//

pub fn and_16(a: Bit16, b: Bit16) -> Bit16 {
  let mut out = [0; 16];
  for (pos, (bit_a, bit_b)) in a.into_iter().zip(b.into_iter()).enumerate() {
    out[pos] = and(bit_a, bit_b);
  }
  return out;
}

// 16-bit OR
//
// a  = 1 | 0 | 1 | 0 | 1 | ....
// b  = 0 | 1 | 1 | 0 | 0 | ....
//    ----+---+---+---+---+
// -> = 1 | 1 | 1 | 1 | 1 | ....
//
pub fn or_16(a: Bit16, b: Bit16) -> Bit16 {
  let mut out = [0; 16];
  for (pos, (bit_a, bit_b)) in a.into_iter().zip(b.into_iter()).enumerate() {
    out[pos] = or(bit_a, bit_b);
  }
  return out;
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

// Multi-way gates (or_8_way, mux_4_way_16, mux_8_way_16, dmux_4_way, dmux_8_way)
//
//

pub fn or_8_way() {}

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
    _ => panic!("error: sel is not valid!"),
  }
}

pub fn mux_8_way_16() {}

pub fn dmux_4_way() {}

pub fn dmux_8_way() {}

// add gates
//
