pub mod ram64_test {
  use nand_computer::{
    gates::Bit16,
    mem::{self},
  };

  /// test writing and reading from a single address
  #[test]
  fn test_ram64_write_and_read() {
    let mut ram = mem::ram64::RAM64::default();

    let address = [1, 0, 1, 0, 1, 0]; // address 42
    let input: Bit16 = [0, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1]; // value 28345
    let load: u8 = 1; // write enabled

    // write input to the RAM
    ram.tick(address, load, input);

    // read back from the same address
    let output = ram.tick(address, 0, [0; 16]);

    // verify the output matches the input
    assert_eq!(output, input, "value at address 42 should match input");
  }

  /// test that writing is not performed when load is disabled
  #[test]
  fn test_ram64_no_write() {
    let mut ram = mem::ram64::RAM64::default();

    let address = [0, 1, 0, 0, 1, 0]; // address 18
    let input: Bit16 = [1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1]; // value 38309
    let load: u8 = 0; // write disabled

    // attempt to write to the RAM (load is 0)
    ram.tick(address, load, input);

    // read back from the same address
    let output = ram.tick(address, 0, [0; 16]);

    // output should still be the default value (zeroes) because write was disabled
    assert_eq!(output, [0; 16], "value at address 18 should remain default (zeroes)");
  }

  /// test writing to multiple addresses and ensuring isolation
  #[test]
  fn test_ram64_multiple_addresses() {
    let mut ram = mem::ram64::RAM64::default();

    let address1 = [0, 0, 1, 0, 0, 1]; // address 9
    let address2 = [1, 0, 0, 1, 0, 0]; // address 36
    let input1: Bit16 = [1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1]; // value 56329
    let input2: Bit16 = [0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0]; // value 15388

    // write to address 9
    ram.tick(address1, 1, input1);

    // write to address 36
    ram.tick(address2, 1, input2);

    // read back from address 9
    let output1 = ram.tick(address1, 0, [0; 16]);

    // read back from address 36
    let output2 = ram.tick(address2, 0, [0; 16]);

    // verify outputs match inputs
    assert_eq!(output1, input1, "value at address 9 should match input1");
    assert_eq!(output2, input2, "value at address 36 should match input2");
  }

  /// test overwriting a value at a single address
  #[test]
  fn test_ram64_overwrite() {
    let mut ram = mem::ram64::RAM64::default();

    let address = [0, 1, 1, 0, 1, 1]; // address 27
    let input1: Bit16 = [0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1]; // first value 10885
    let input2: Bit16 = [1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0]; // second value 39844

    // write the first value to the RAM
    ram.tick(address, 1, input1);

    // overwrite with the second value
    ram.tick(address, 1, input2);

    // read back the value
    let output = ram.tick(address, 0, [0; 16]);

    // verify the output matches the second input
    assert_eq!(output, input2, "value at address 27 should match input2");
  }
}
