namespace Money.Tests;

using NUnit.Framework;

using Money;

public class Test
{
    [SetUp]
    public void Setup()
    {
    }

    [Test]
    public void TestAddMultiplication()
    {
        Dollar product;
        var five = new Dollar(5);

        product = five.times(2);
        Assert.That(product.amount, Is.EqualTo(10));

        product = five.times(3);
        Assert.That(product.amount, Is.EqualTo(15));
    }
}
