#[cfg(test)]
mod tests {
  use nand_computer::{gates::Bit16, mem::ram512::RAM512};

  #[test]
  fn ram512_write_and_read() {
    // create ram512
    let mut ram512 = RAM512::default();

    // write a number to address 0 (e.g., register holding an operand)
    let address = [0, 0, 0, 0, 0, 0, 0, 0, 0]; // binary 0
    let input: Bit16 = [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1]; // value 1234
    ram512.tick(address, 1, input);

    // read the number back from address 0
    let output = ram512.tick(address, 0, [0; 16]);
    assert_eq!(output, input, "value at address 0 should be 12345");

    // write another number to address 255 (middle memory location)
    let address = [0, 1, 1, 1, 1, 1, 1, 1, 1]; // binary 255
    let input: Bit16 = [1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1]; // example value 43690
    ram512.tick(address, 1, input);

    // read the number back from address 255
    let output = ram512.tick(address, 0, [0; 16]);
    assert_eq!(output, input, "value at address 255 should be 43690");

    // write a different value to address 511 (last memory location)
    let address = [1, 1, 1, 1, 1, 1, 1, 1, 1]; // binary 511
    let input: Bit16 = [0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0]; // example value 21930
    ram512.tick(address, 1, input);

    // read the number back from address 511
    let output = ram512.tick(address, 0, [0; 16]);
    assert_eq!(output, input, "value at address 511 should be 21930");

    // ensure value at address 0 is not affected
    let address = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let output = ram512.tick(address, 0, [0; 16]);
    assert_eq!(output, [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1], "value at address 0 should still be 12345");

    // ensure value at address 255 is not affected
    let address = [0, 1, 1, 1, 1, 1, 1, 1, 1];
    let output = ram512.tick(address, 0, [0; 16]);
    assert_eq!(output, [1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1], "value at address 255 should still be 43690");
  }
}
