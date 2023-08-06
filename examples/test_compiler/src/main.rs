extern crate mico_ir;

fn main() {
  let ir = std::fs::read_to_string("example_ir.txt").unwrap();
  mico_ir::gen(ir);
}
