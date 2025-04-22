use super::*;

pub struct Analyzer<'a> {
  environment: Environment<'a>,
}

impl<'a> Analyzer<'a> {
  pub fn new(environment: Environment<'a>) -> Self {
    Self { environment }
  }

  pub fn analyze(&mut self, ast: &Spanned<Program<'a>>) -> Vec<Error> {
    Vec::new()
  }
}
