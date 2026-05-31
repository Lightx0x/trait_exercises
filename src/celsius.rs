// exercise 1 — Celsius newtype
// build a Celsius struct that wraps an f64.
// 
// construct from f64, i32, and &f64 via From
// support + and - between two Celsius values
// Support + with an f64 on the right
// Compare two Celsius values for equality
// Print with debug format
// Reusable in expressions like a + b + c

use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Celsius {
    pub value: f64,
}

impl Celsius {
    fn new(value: f64) -> Self {
        Celsius { value }
    }
}

impl From<f64> for Celsius {
    fn from(value: f64) -> Self {
        Celsius::new(value)
    }
}

impl From<&f64> for Celsius {
    fn from(value: &f64) -> Self {
        Celsius::new(*value)
    }
}

impl From<i32> for Celsius {
    fn from(value: i32) -> Self {
        Celsius::new(value.into())
    }
}

impl Add<Celsius> for Celsius {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Celsius::new(self.value + rhs.value) 
    }
}

impl Sub<Celsius> for Celsius {
    type Output = Self;

    fn sub(self, rhs: Celsius) -> Self::Output {
        Celsius::new(self.value - rhs.value)
    }
}

impl Add<f64> for Celsius {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Celsius::new(self.value.add(rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius() {
        let a = Celsius::new(10.0);
        let b = Celsius::new(20.0);
        let c = Celsius::new(5.0);
        
        assert_eq!(a + b + c, Celsius::new(35.0));
        assert_eq!(b - a, Celsius::new(10.0));
        assert_eq!(a + 5.0, Celsius::new(15.0));
        
        let from_int: Celsius = 25i32.into();
        let from_ref: Celsius = (&3.5f64).into();
        assert_eq!(from_int, Celsius::new(25.0));
        assert_eq!(from_ref, Celsius::new(3.5));
    }
}
