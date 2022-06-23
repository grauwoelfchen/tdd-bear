use crate::expression::Expression;
use crate::bank::Bank;
use crate::money::Money;
use crate::sum::Sum;

pub struct Mul<'a> {
  multiplicand: &'a (dyn Expression + 'a),
  multiplier: u32,
}

impl<'a> Mul<'a> {
  pub fn new(
    multiplicand: &'a (dyn Expression + 'a),
    multiplier: u32,
  ) -> Mul<'a> {
    Self {
      multiplicand,
      multiplier,
    }
  }
}

impl<'b> Expression for Mul<'b> {
  fn plus<'a>(&'a self, addend: &'a (dyn Expression + 'a)) -> Sum<'a> {
    Sum::new(self, addend)
  }

  fn reduce(&self, bank: &Bank, to: &'static str) -> Money {
    let m = self.multiplicand.reduce(bank, to);
    Money::new(m.amount() * self.multiplier, m.currency())
  }

  fn times<'a>(&'a self, multiplier: u32) -> Mul<'a> {
    Mul::new(self, multiplier)
  }
}
