use core::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Currency {
    Dollar,
    Franc,
}

trait Expression {
    fn reduce(&self, to: Currency) -> Money;
}

#[derive(Debug, PartialEq)]
pub struct Money(u32, Currency);

impl Money {
    pub fn dollar(amount: u32) -> Money {
        Money(amount, Currency::Dollar)
    }
    pub fn franc(amount: u32) -> Money {
        Money(amount, Currency::Franc)
    }
    pub fn times(&self, multiplier: u32) -> Money {
        let product = self.0 * multiplier;
        match self.1 {
            Currency::Franc => Money(product, Currency::Franc),
            Currency::Dollar => Money(product, Currency::Dollar),
        }
    }
    // TODO: use operator overloading
    // Returning a trait ? Or a Box<dyn Trait> ?
    fn plus(&self, operand: &Money) -> impl Expression {
        Money(self.0 + operand.0, self.1)
    }
}

impl Expression for Money {
    fn reduce(&self, to: Currency) -> Money {
        Money(self.0, self.1)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let currency_str = match self.1 {
            Currency::Franc => "CHF",
            Currency::Dollar => "$",
        };
        f.write_fmt(format_args!("{}{}", currency_str, self.0))
    }
}

struct Bank;

impl Bank {
    fn new() -> Bank {
        Bank {}
    }
    fn reduce(&self, source: impl Expression, to: Currency) -> Money {
        source.reduce(to)
    }
}

struct Sum(Money, Money);

impl Expression for Sum {
    fn reduce(&self, to: Currency) -> Money {
        Money(self.0.0 + self.1.0, to)
    }
}

#[cfg(test)]
mod test {
    use crate::currency::{Bank, Currency, Money, Sum};

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::dollar(6));
        assert_eq!(Money::franc(5), Money::franc(5));
        assert_ne!(Money::franc(5), Money::franc(6));
        assert_ne!(Money::franc(5), Money::dollar(5));
    }

    // Can't implement in Rust
    // #[test]
    // fn test_plus_return_sum() {
    //     let five = Money::dollar(5);
    //     let result = five.plus(&five);
    //     assert_eq!(five, sum.augend);
    //     assert_eq!(five, sum.addend);
    // }

    #[test]
    fn test_reduce_money() {
        let money = Money::dollar(7);
        let bank = Bank::new();
        let reduced = bank.reduce(money, Currency::Dollar);
        assert_eq!(reduced, Money::dollar(7));
    }

    #[test]
    fn test_reduce_sum() {
        let sum = Sum(Money::dollar(3), Money::dollar(4));
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Currency::Dollar);
        assert_eq!(reduced, Money::dollar(7));
    }

    #[test]
    fn test_simple_addition() {
        let five = Money::dollar(5);
        let sum = five.plus(&five);
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Currency::Dollar);
        assert_eq!(reduced, Money::dollar(10));
    }
}
