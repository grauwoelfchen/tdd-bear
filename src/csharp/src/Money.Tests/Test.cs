namespace Money.Tests;

using NUnit.Framework;

using Money;

public class Test {
  [SetUp]
  public void Setup() {
  }

  [Test]
  public void TestAddMultiplication() {
    Dollar product;
    var five = new Dollar(5);

    product = five.Times(2);
    Assert.That(product.amount, Is.EqualTo(10));

    product = five.Times(3);
    Assert.That(product.amount, Is.EqualTo(15));
  }

  [Test]
  public void TestEquality() {
    Assert.True(new Dollar(5).Equals(new Dollar(5)));
    Assert.False(new Dollar(5).Equals(new Dollar(6)));
  }
}
