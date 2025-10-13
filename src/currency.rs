use core::fmt;

#[derive(Debug, PartialEq, Eq)]
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
    use crate::currency::Dollar;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert_eq!(Dollar::new(10), five.times(2));
        assert_eq!(Dollar::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_eq!(Dollar::new(5), Dollar::new(5));
        assert_ne!(Dollar::new(5), Dollar::new(6));
    }
}
