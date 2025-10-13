use core::fmt;

#[derive(Debug, PartialEq)]
enum Currency {
    Dollar,
    Franc,
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

#[cfg(test)]
mod test {
    use crate::currency::Money;

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
}
