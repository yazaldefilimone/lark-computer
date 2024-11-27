#[cfg(test)]
mod gates_test {
  use nand_computer::gates::{self, Bit16};
  #[test]
  fn test_nand() {
    assert_eq!(gates::nand(0, 0), 1);
    assert_eq!(gates::nand(0, 1), 1);
    assert_eq!(gates::nand(1, 0), 1);
    assert_eq!(gates::nand(1, 1), 0);
  }

  #[test]
  fn test_not() {
    assert_eq!(gates::not(0), 1);
    assert_eq!(gates::not(1), 0);
  }

  #[test]
  fn test_and() {
    assert_eq!(gates::and(0, 0), 0);
    assert_eq!(gates::and(0, 1), 0);
    assert_eq!(gates::and(1, 0), 0);
    assert_eq!(gates::and(1, 1), 1);
  }

  #[test]
  fn test_or() {
    assert_eq!(gates::or(0, 0), 0);
    assert_eq!(gates::or(0, 1), 1);
    assert_eq!(gates::or(1, 0), 1);
    assert_eq!(gates::or(1, 1), 1);
  }

  #[test]
  fn test_xor() {
    assert_eq!(gates::xor(0, 0), 0);
    assert_eq!(gates::xor(0, 1), 1);
    assert_eq!(gates::xor(1, 0), 1);
    assert_eq!(gates::xor(1, 1), 0);
  }

  #[test]
  fn test_mux() {
    assert_eq!(gates::mux(0, 1, 0), 0);
    assert_eq!(gates::mux(0, 1, 1), 1);
  }

  #[test]
  fn test_dmux() {
    assert_eq!(gates::dmux(1, 0), (1, 0));
    assert_eq!(gates::dmux(1, 1), (0, 1));
  }

  #[test]
  fn test_mux16() {
    let a = [0; 16];
    let b = [1; 16];
    assert_eq!(gates::mux16(a, b, 0), a);
    assert_eq!(gates::mux16(a, b, 1), b);
  }

  #[test]
  fn test_mux4way16() {
    let a = [0; 16];
    let b = [1; 16];
    let c = [0; 16];
    let d = [1; 16];
    assert_eq!(gates::mux4way16(a, b, c, d, [0, 0]), a);
    assert_eq!(gates::mux4way16(a, b, c, d, [1, 0]), b);
    assert_eq!(gates::mux4way16(a, b, c, d, [0, 1]), c);
    assert_eq!(gates::mux4way16(a, b, c, d, [1, 1]), d);
  }

  #[test]
  fn test_mux8way16() {
    let a = [0; 16];
    let b = [1; 16];
    let c = [0; 16];
    let d = [1; 16];
    let e = [0; 16];
    let f = [1; 16];
    let g = [0; 16];
    let h = [1; 16];
    assert_eq!(gates::mux8way16(a, b, c, d, e, f, g, h, [0, 0, 0]), a);
    assert_eq!(gates::mux8way16(a, b, c, d, e, f, g, h, [1, 0, 1]), f);
  }

  #[test]
  fn test_dmux8way() {
    assert_eq!(gates::dmux8way(1, [0, 0, 0]), (1, 0, 0, 0, 0, 0, 0, 0));
    assert_eq!(gates::dmux8way(1, [0, 1, 0]), (0, 0, 1, 0, 0, 0, 0, 0));
  }

  #[test]
  fn test_half_adder() {
    assert_eq!(gates::half_adder(0, 0), (0, 0)); // 0 + 0 = 0, carry = 0
    assert_eq!(gates::half_adder(0, 1), (1, 0)); // 0 + 1 = 1, carry = 0
    assert_eq!(gates::half_adder(1, 0), (1, 0)); // 1 + 0 = 1, carry = 0
    assert_eq!(gates::half_adder(1, 1), (0, 1)); // 1 + 1 = 0, carry = 1
  }

  #[test]
  fn test_full_adder() {
    assert_eq!(gates::full_adder(0, 0, 0), (0, 0)); // 0 + 0 + 0 = 0, carry = 0
    assert_eq!(gates::full_adder(0, 1, 0), (1, 0)); // 0 + 1 + 0 = 1, carry = 0
    assert_eq!(gates::full_adder(1, 0, 0), (1, 0)); // 1 + 0 + 0 = 1, carry = 0
    assert_eq!(gates::full_adder(1, 1, 0), (0, 1)); // 1 + 1 + 0 = 0, carry = 1
    assert_eq!(gates::full_adder(1, 1, 1), (1, 1)); // 1 + 1 + 1 = 1, carry = 1
  }

  #[test]
  fn test_add_16() {
    let a: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // 1
    let b: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // 1
    let expected: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]; // 2
    assert_eq!(gates::add_16(a, b), expected);
  }

  #[test]
  fn test_and_16() {
    let a: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // 1
    let b: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // 1
    let expected: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // 1
    assert_eq!(gates::and_16(a, b), expected);

    // case 2
    let a: Bit16 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]; // 0b11111111
    let b: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // 0b11111111
    let expected: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // 0b11111111
    assert_eq!(gates::and_16(a, b), expected);
  }

  #[test]
  fn test_not_16() {
    let a: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // 0b11111111
    let expected: Bit16 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0]; // 0b11111111
    assert_eq!(gates::not_16(a), expected);

    // case 2
    let a: Bit16 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]; // 0b11111111
    let expected: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // 0b11111111
    assert_eq!(gates::not_16(a), expected);
  }

  #[test]
  fn test_inc_16() {
    let a: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // 1
    let expected: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]; // 2
    assert_eq!(gates::inc_16(a), expected);
  }
}
