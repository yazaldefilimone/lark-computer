#![allow(dead_code)]

use crate::gates::{self, Bit, Bit16};

pub fn half_adder(a: Bit, b: Bit) -> (Bit, Bit) {
  let sum = gates::xor(a, b);
  let carry = gates::and(a, b);
  (sum, carry)
}

pub fn full_adder(a: Bit, b: Bit, carry_in: Bit) -> (Bit, Bit) {
  let (sum_one, carry_one) = half_adder(a, b);
  let (sum_two, carry_two) = half_adder(sum_one, carry_in);
  (sum_two, gates::or(carry_one, carry_two))
}

// // 16-bit ALU
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
