use crate::expression::Expression;
use crate::bank::Bank;
use crate::money::Money;
use crate::mul::Mul;

pub struct Sum<'a> {
  augend: &'a (dyn Expression + 'a),
  addend: &'a (dyn Expression + 'a),
}

impl<'a> Sum<'a> {
  pub fn new(
    augend: &'a (dyn Expression + 'a),
    addend: &'a (dyn Expression + 'a),
  ) -> Sum<'a> {
    Self { augend, addend }
  }
}

impl<'b> Expression for Sum<'b> {
  fn plus<'a>(&'a self, addend: &'a (dyn Expression + 'a)) -> Sum<'a> {
    Sum::new(self, addend)
  }

  fn reduce(&self, bank: &Bank, to: &'static str) -> Money {
    let amount = self.augend.reduce(bank, to).amount()
      + self.addend.reduce(bank, to).amount();
    Money::new(amount, to)
  }

  fn times<'a>(&'a self, multiplier: u32) -> Mul<'a> {
    Mul::new(self, multiplier)
  }
}
