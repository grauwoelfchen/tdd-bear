use crate::money::Money;
use crate::bank::Bank;
use crate::mul::Mul;
use crate::sum::Sum;

pub trait Expression {
  fn plus<'a>(&'a self, addend: &'a (dyn Expression + 'a)) -> Sum<'a>;
  fn times<'a>(&'a self, multiplier: u32) -> Mul<'a>;
  fn reduce(&self, bank: &Bank, to: &'static str) -> Money;
}
