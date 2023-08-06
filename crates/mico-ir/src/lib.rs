mod lexer;
mod token;

/// Generates an object file from the IR provided
pub fn gen(ir: String) {
  let lexer = lexer::Lexer::new(ir);

  for t in lexer.get_tkns() {
    println!("{}: {}", t.name, t.val);
  }
}
