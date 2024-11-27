#[cfg(test)]
mod mem_test {
  use nand_computer::{
    gates::Bit16,
    mem::{self},
  };

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

  // 8-bits RAM cases test
  #[test]
  fn test_ram8_write_and_read() {
    let mut ram = mem::ram8::RAM8::default();

    let address = [1, 0, 1]; // address 5
    let input: Bit16 = [1; 16]; // input all ones
    let load: u8 = 1; // write enabled

    // write input to the RAM
    ram.tick(address, load, input);

    // read back from the same address
    let output = ram.tick(address, 0, [0; 16]);

    // Verify the output matches the input
    assert_eq!(output, input);
  }

  #[test]
  fn test_ram8_no_write() {
    let mut ram = mem::ram8::RAM8::default();

    let address = [0, 1, 0]; // address 2
    let input: Bit16 = [1; 16]; // input all ones
    let load: u8 = 0; // write disabled

    // attempt to write to the RAM (but load is 0)
    ram.tick(address, load, input);

    // read back from the same address
    let output = ram.tick(address, 0, [0; 16]);

    // output should still be the default value (zeroes) because write was disabled
    assert_eq!(output, [0; 16]);
  }

  #[test]
  fn test_ram8_multiple_addresses() {
    let mut ram = mem::ram8::RAM8::default();

    let address1 = [0, 0, 1]; // address 1
    let address2 = [1, 0, 0]; // address 4
    let input1: Bit16 = [1; 16]; // input all ones
    let input2: Bit16 = [0; 16]; // input all zeroes

    // write to address 1
    ram.tick(address1, 1, input1);

    // write to address 4
    ram.tick(address2, 1, input2);

    // read back from address 1
    let output1 = ram.tick(address1, 0, [0; 16]);

    // read back from address 4
    let output2 = ram.tick(address2, 0, [0; 16]);

    // Verify outputs
    assert_eq!(output1, input1);
    assert_eq!(output2, input2);
  }

  #[test]
  fn test_ram8_overwrite() {
    let mut ram = mem::ram8::RAM8::default();

    let address = [0, 1, 1]; // address 3
    let input1: Bit16 = [1; 16]; // first input
    let input2: Bit16 = [0; 16]; // second input

    // write the first value to the RAM
    ram.tick(address, 1, input1);

    // overwrite with the second value
    ram.tick(address, 1, input2);

    // read back the value
    let output = ram.tick(address, 0, [0; 16]);

    // verify the output matches the second input
    assert_eq!(output, input2);
  }

  // 64-bits RAM cases test
  #[test]
  fn test_ram64_write_and_read() {
    let mut ram = mem::ram64::RAM64::default();

    let address = [1, 0, 1, 0, 1, 0]; // address 63
    let input: Bit16 = [1; 16]; // input all ones
    let load: u8 = 1; // write enabled

    // write input to the RAM
    ram.tick(address, load, input);

    // read back from the same address
    let output = ram.tick(address, 0, [0; 16]);

    // Verify the output matches the input
    assert_eq!(output, input);
  }

  #[test]
  fn test_ram64_no_write() {
    let mut ram = mem::ram64::RAM64::default();

    let address = [0, 1, 0, 0, 1, 0]; // address 32
    let input: Bit16 = [1; 16]; // input all ones
    let load: u8 = 0; // write disabled

    // attempt to write to the RAM (but load is 0)
    ram.tick(address, load, input);

    // read back from the same address
    let output = ram.tick(address, 0, [0; 16]);

    // output should still be the default value (zeroes) because write was disabled
    assert_eq!(output, [0; 16]);
  }

  #[test]
  fn test_ram64_multiple_addresses() {
    let mut ram = mem::ram64::RAM64::default();

    let address1 = [0, 0, 1, 0, 0, 1]; // address 1
    let address2 = [1, 0, 0, 1, 0, 0]; // address 32
    let input1: Bit16 = [1; 16]; // input all ones
    let input2: Bit16 = [0; 16]; // input all zeroes

    // write to address 1
    ram.tick(address1, 1, input1);

    // write to address 32
    ram.tick(address2, 1, input2);

    // read back from address 1
    let output1 = ram.tick(address1, 0, [0; 16]);

    // read back from address 32
    let output2 = ram.tick(address2, 0, [0; 16]);

    // Verify outputs
    assert_eq!(output1, input1);
    assert_eq!(output2, input2);
  }

  #[test]
  fn test_ram64_overwrite() {
    let mut ram = mem::ram64::RAM64::default();

    let address = [0, 1, 1, 0, 1, 1]; // address 33
    let input1: Bit16 = [1; 16]; // first input
    let input2: Bit16 = [0; 16]; // second input

    // write the first value to the RAM
    ram.tick(address, 1, input1);

    // overwrite with the second value
    ram.tick(address, 1, input2);

    // read back the value
    let output = ram.tick(address, 0, [0; 16]);

    // verify the output matches the second input
    assert_eq!(output, input2);
  }
}
