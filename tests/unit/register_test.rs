#[cfg(test)]
mod mem_test {
  use nand_computer::mem::{self};

  // DFF cases test
  #[test]
  fn test_dff() {
    let mut dff = mem::register::DFF::default();

    // tick the DFF
    dff.tick(1);

    // read the output
    let output = dff.output();

    // verify the output
    assert_eq!(output, 1);
  }

  // Bit cases test
  #[test]
  fn test_bit() {
    let mut bit = mem::register::Bit::default();

    // tick the Bit
    bit.tick(1, 0);

    // read the output
    let output = bit.output();

    // verify the output
    assert_eq!(output, 0);
  }

  #[test]
  fn test_bit_load() {
    let mut bit = mem::register::Bit::default();

    // tick the Bit
    bit.tick(1, 1);

    // read the output
    let output = bit.output();

    // verify the output
    assert_eq!(output, 1);
  }
}
