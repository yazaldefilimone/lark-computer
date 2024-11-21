pub type Bin = u8;

pub fn nand(a: Bin, b: Bin) -> Bin {
  !(a & b) & 1
}

pub fn not(a: Bin) -> Bin {
  nand(a, 1)
}

pub fn and(a: Bin, b: Bin) -> Bin {
  nand(nand(a, b), nand(a, b))
}

pub fn or(a: Bin, b: Bin) -> Bin {
  nand(not(a), not(b))
}

pub fn xor(a: Bin, b: Bin) -> Bin {
  let not_a_and_b = nand(nand(a, 1), b);
  let a_and_not_b = nand(a, nand(b, 1));
  nand(not_a_and_b, a_and_not_b)
  // nand(nand(a, nand(a, b)), nand(b, nand(a, b)))
}

pub fn mux(a: Bin, b: Bin, sel: Bin) -> Bin {
  (a & !sel) | (b & sel)
}

pub fn dmux(input: Bin, sel: Bin) -> (Bin, Bin) {
  let a = input & !sel;
  let b = input & sel;
  (a, b)
}
