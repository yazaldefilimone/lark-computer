#[cfg(test)]
mod tests {
  use nand_computer::{
    alu::ALU,
    gates::{self, Bit16},
  };

  #[test]
  fn test_zero_x() {
    // input x = 1111111111111111
    // input y = 0000000000000000
    let x: Bit16 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let y: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut alu = ALU::new(x, y);

    // control zx = 1 -> zero the x input
    alu.set_zx(1);

    let (out, zr, ng) = alu.execute();

    // expected out = 0000000000000000
    assert_eq!(out, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    // expected zr = 1 (output is zero)
    assert_eq!(zr, 1);
    // expected ng = 0 (output is not negative)
    assert_eq!(ng, 0);
  }

  #[test]
  fn test_negate_x() {
    // input x = 0000000000000000
    // input y = 1111111111111111
    let x: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let y: Bit16 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let mut alu = ALU::new(x, y);

    // control zx = 1, nx = 1 -> zero and negate the x input
    alu.set_zx(1);
    alu.set_nx(1);

    let (out, zr, ng) = alu.execute();

    // expected out = 1111111111111111
    assert_eq!(out, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    // expected zr = 0 (output is not zero)
    assert_eq!(zr, 0);
    // expected ng = 1 (output is negative in two's complement)
    assert_eq!(ng, 1);
  }

  #[test]
  fn test_addition() {
    // input x = 0000000000000001 (1)
    // input y = 0000000000000001 (1)
    let x: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let y: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let mut alu = ALU::new(x, y);

    // control f = 1 -> add x and y
    alu.set_f(1);

    let (out, zr, ng) = alu.execute();

    // expected out = 0000000000000010 (2)
    assert_eq!(out, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]);
    // expected zr = 0 (output is not zero)
    assert_eq!(zr, 0);
    // expected ng = 0 (output is not negative)
    assert_eq!(ng, 0);
  }

  #[test]
  fn test_and() {
    // input x = 1111111111111111 (1)
    // input y = 1010101010101010 (1010)
    let x: Bit16 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let y: Bit16 = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
    let mut alu = ALU::new(x, y);

    // control f = 0 -> perform AND operation
    alu.set_f(0);

    let (out, zr, ng) = alu.execute();

    // expected out = 1010101010101010 (1010)
    assert_eq!(out, [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]);
    // expected zr = 0 (output is not zero)
    assert_eq!(zr, 0);
    // expected ng = 0 (output is not negative)
    assert_eq!(ng, 1);
  }

  #[test]
  fn test_negate_output() {
    // input x = 1111111111111111
    // input y = 0000000000000000
    let x: Bit16 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let y: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut alu = ALU::new(x, y);

    // control f = 0, no = 1 -> AND x, y, then negate
    alu.set_f(0);
    alu.set_no(1);

    let (out, zr, ng) = alu.execute();

    // expected out = 1111111111111111 (NOT of AND x, y)
    assert_eq!(out, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    // expected zr = 0 (output is not zero)
    assert_eq!(zr, 0);
    // expected ng = 0 (output is not negative)
    assert_eq!(ng, 1);
  }

  // ----------
  #[test]
  fn test_all_alu_cases() {
    let x: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // x = 1
    let y: Bit16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]; // y = 2
    let mut alu = ALU::new(x, y);

    alu.set_controls([1, 0, 1, 0, 1, 0]);
    let expected = [0; 16];
    let (out, zr, ng) = alu.execute();
    assert_eq!(out, expected);
    assert_eq!(zr, 1);
    assert_eq!(ng, 0);

    alu = ALU::new(x, y);
    alu.set_controls([1, 1, 1, 1, 1, 1]);
    let expected = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let (out, zr, ng) = alu.execute();
    assert_eq!(out, expected);
    assert_eq!(zr, 0);
    assert_eq!(ng, 0);

    alu = ALU::new(x, y);
    alu.set_controls([1, 1, 1, 0, 1, 0]);
    let expected = [1; 16];
    let (out, zr, ng) = alu.execute();
    assert_eq!(out, expected);
    assert_eq!(zr, 0);
    assert_eq!(ng, 1);

    alu = ALU::new(x, y);
    alu.set_controls([0, 0, 1, 1, 0, 0]);
    let expected = x;
    let (out, zr, ng) = alu.execute();
    alu.show();
    assert_eq!(out, expected);
    assert_eq!(zr, 0);
    assert_eq!(ng, 0);

    alu = ALU::new(x, y);
    alu.set_controls([1, 1, 0, 0, 0, 0]);
    let expected = y;
    let (out, zr, ng) = alu.execute();
    assert_eq!(out, expected);
    assert_eq!(zr, 0);
    assert_eq!(ng, 0);

    alu = ALU::new(x, y);
    alu.set_controls([0, 0, 1, 1, 0, 1]);
    let expected = gates::not_16(x);
    let (out, zr, ng) = alu.execute();
    assert_eq!(out, expected);
    assert_eq!(zr, 0);
    assert_eq!(ng, 1);

    alu = ALU::new(x, y);
    alu.set_controls([1, 1, 0, 0, 0, 1]);
    let expected = gates::not_16(y);
    let (out, zr, ng) = alu.execute();
    assert_eq!(out, expected);
    assert_eq!(zr, 0);
    assert_eq!(ng, 1);

    alu = ALU::new(x, y);
    alu.set_controls([0, 0, 1, 1, 1, 1]);
    let expected = gates::add_16(gates::not_16(x), [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    let (out, zr, ng) = alu.execute();
    assert_eq!(out, expected);
    assert_eq!(zr, 0);
    assert_eq!(ng, 1);
  }
}
