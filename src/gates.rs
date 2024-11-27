pub type Bit = u8;
pub type Bit4 = [Bit; 4];
pub type Bit8 = [Bit; 8];
pub type Bit16 = [Bit; 16];

fn eq_bit(a: Bit, b: Bit) -> Bit {
  not(xor(a, b))
}

// NAND GATE (BASE)
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
  not(nand(a, b))
}

// OR GATE
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
// a  | b  | ->
// ---+----+----
//  0 |  0 |  0
//  0 |  1 |  1
//  1 |  0 |  1
//  1 |  1 |  0
pub fn xor(a: Bit, b: Bit) -> Bit {
  // let not_a_and_b = nand(nand(a, 1), b);
  // let a_and_not_b = nand(a, nand(b, 1));
  // nand(not_a_and_b, a_and_not_b)
  nand(nand(a, nand(a, b)), nand(b, nand(a, b)))
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
  for i in 0..16 {
    out[i] = and(a[i], b[i]);
  }
  out
}

// 16-bit OR
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

pub fn or_8_way(n: Bit8) -> Bit8 {
  let mut out = [0; 8];
  for (pos, bit) in n.into_iter().enumerate() {
    out[pos] = or(bit, out[pos]);
  }
  return out;
}

/*
 | a  | b  |  sum  | carry
 +----+----+-------+-------
 |  0 |  0 |   0  |  0  |
 |  0 |  1 |   0  |  0  |
 |  1 |  0 |   0  |  0  |
 |  1 |  1 |   1  |  1  |
*/
pub fn half_adder(a: Bit, b: Bit) -> (Bit, Bit) {
  let sum = xor(a, b);
  let carry = and(a, b);
  (sum, carry)
}

/*
 | a  | b  | c |  sum | carry
 +----+----+---+------+-------
 |  0 |  0 | 0 |   0  |  0  |
 |  0 |  1 | 0 |   0  |  0  |
 |  0 |  0 | 1 |   0  |  0  |
 |  0 |  1 | 1 |   0  |  0  |
 |  1 |  0 | 0 |   0  |  0  |
 |  1 |  1 | 0 |   0  |  0  |
 |  1 |  0 | 1 |   0  |  0  |
 |  1 |  1 | 1 |   1  |  1  |
*/
pub fn full_adder(a: Bit, b: Bit, c: Bit) -> (Bit, Bit) {
  let (sum_one, carry_one) = half_adder(a, b);
  let (sum_two, carry_two) = half_adder(sum_one, c);
  (sum_two, or(carry_one, carry_two))
}

pub fn add_16(a: Bit16, b: Bit16) -> Bit16 {
  let mut out = [0; 16];
  let mut carry = 0;
  for pos in (0..16).rev() {
    let (sum, new_carry) = full_adder(a[pos], b[pos], carry);
    out[pos] = sum;
    carry = new_carry;
  }
  return out;
}

pub fn inc_16(a: Bit16) -> Bit16 {
  let mut out = [0; 16];
  let mut carry = 1;
  for pos in (0..16).rev() {
    let (sum, new_carry) = full_adder(a[pos], 0, carry);
    out[pos] = sum;
    carry = new_carry;
  }
  out
}

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
pub fn mux16(a: Bit16, b: Bit16, sel: Bit) -> Bit16 {
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

pub fn mux4way16(a: Bit16, b: Bit16, c: Bit16, d: Bit16, sel: Bit2) -> Bit16 {
  match sel {
    [0, 0] => a,
    [1, 0] => b,
    [0, 1] => c,
    [1, 1] => d,
    _ => panic!("error: sel value is not valid!"),
  }
}

pub fn mux8way16(a: Bit16, b: Bit16, c: Bit16, d: Bit16, e: Bit16, f: Bit16, g: Bit16, h: Bit16, sel: Bit3) -> Bit16 {
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

pub fn dmux4way(n: Bit, sel: Bit2) -> (Bit, Bit, Bit, Bit) {
  match sel {
    [0, 0] => (n, 0, 0, 0),
    [1, 0] => (0, n, 0, 0),
    [0, 1] => (0, 0, n, 0),
    [1, 1] => (0, 0, 0, n),
    _ => panic!("error: sel value is not valid!"),
  }
}

pub fn dmux8way(n: Bit, sel: Bit3) -> (Bit, Bit, Bit, Bit, Bit, Bit, Bit, Bit) {
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
