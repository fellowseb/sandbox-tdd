use core::fmt;

pub struct Dollar {
    amount: u32,
}

impl Dollar {
    pub fn new(amount: u32) -> Dollar {
        Dollar { amount: amount }
    }
    pub fn times(&self, multiplier: u32) -> Self {
        Dollar {
            amount: self.amount * multiplier,
        }
    }
}

impl fmt::Display for Dollar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("${}", self.amount))
    }
}

#[cfg(test)]
mod test {
    use crate::currency_ops::Dollar;

    #[test]
    fn test_multiplication() {
        let five = Dollar { amount: 5 };
        let mut product = five.times(2);
        assert_eq!(10, product.amount);
        product = five.times(3);
        assert_eq!(15, product.amount);
    }
}
