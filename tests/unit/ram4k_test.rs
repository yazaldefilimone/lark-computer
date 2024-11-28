pub mod ram4k_test {
  use nand_computer::{
    gates::Bit16,
    mem::{self},
  };

  /// test writing and reading from a single address
  #[test]
  fn test_ram4k_write_and_read() {
    let mut ram = mem::ram4k::RAM4K::default();

    let address = [0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0]; // address 444
    let input: Bit16 = [1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1]; // value 47979
    let load: u8 = 1; // write enabled

    // write input to the RAM
    ram.tick(address, load, input);

    // read back from the same address
    let output = ram.tick(address, 0, [0; 16]);

    // verify the output matches the input
    assert_eq!(output, input, "value at address 444 should match input");
  }

  /// test that writing is not performed when load is disabled
  #[test]
  fn test_ram4k_no_write() {
    let mut ram = mem::ram4k::RAM4K::default();

    let address = [1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1]; // address 2213
    let input: Bit16 = [0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1]; // value 23721
    let load: u8 = 0; // write disabled

    // attempt to write to the RAM (load is 0)
    ram.tick(address, load, input);

    // read back from the same address
    let output = ram.tick(address, 0, [0; 16]);

    // output should still be the default value (zeroes) because write was disabled
    assert_eq!(output, [0; 16], "value at address 2213 should remain default (zeroes)");
  }

  /// test writing to multiple addresses and ensuring isolation
  #[test]
  fn test_ram4k_multiple_addresses() {
    let mut ram = mem::ram4k::RAM4K::default();

    let address1 = [0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1]; // address 347
    let address2 = [1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0]; // address 3076
    let input1: Bit16 = [1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1]; // value 42355
    let input2: Bit16 = [0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0]; // value 23406

    // write to address 347
    ram.tick(address1, 1, input1);

    // write to address 3076
    ram.tick(address2, 1, input2);

    // read back from address 347
    let output1 = ram.tick(address1, 0, [0; 16]);

    // read back from address 3076
    let output2 = ram.tick(address2, 0, [0; 16]);

    // verify outputs match inputs
    assert_eq!(output1, input1, "value at address 347 should match input1");
    assert_eq!(output2, input2, "value at address 3076 should match input2");
  }

  /// test overwriting a value at a single address
  #[test]
  fn test_ram4k_overwrite() {
    let mut ram = mem::ram4k::RAM4K::default();

    let address = [1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1]; // address 2353
    let input1: Bit16 = [1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0]; // first value 51418
    let input2: Bit16 = [0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1]; // second value 11803

    // write the first value to the RAM
    ram.tick(address, 1, input1);

    // overwrite with the second value
    ram.tick(address, 1, input2);

    // read back the value
    let output = ram.tick(address, 0, [0; 16]);

    // verify the output matches the second input
    assert_eq!(output, input2, "value at address 2353 should match input2");
  }
}
