pub struct Dollar {
    amount: u32,
}

impl Dollar {
    pub fn new(amount: u32) -> Dollar {
        Dollar { amount: amount }
    }
    pub fn times(&mut self, multiplier: u32) -> &Self {
        self.amount *= multiplier;
        self
    }
}

#[cfg(test)]
mod test {
    use crate::currency_ops::Dollar;

    #[test]
    fn test_multiplication() {
        let mut five = Dollar { amount: 5 };
        five.times(2);
        assert_eq!(10, five.amount)
    }
}
