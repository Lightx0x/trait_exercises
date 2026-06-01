// **Exercise 3 — `Money` with currency safety**
// 
// Build two newtype structs representing amounts in cents:
// 
// - `Usd(u64)` — US dollars in cents
// - `Eur(u64)` — Euros in cents
// 
// Requirements:
// - Both support `+` and `-` with their own type only (`Usd + Usd`, `Eur + Eur`)
// - They should **not** be addable to each other — the type system should prevent `usd + eur` from compiling
// - Implement `From<u64>` for both, so you can write `Usd::from(500)` or `500u64.into()`
// - Add a method `to_dollars(&self) -> f64` on `Usd` that converts cents to dollars (divide by 100.0)
// - Add a method `to_euros(&self) -> f64` on `Eur` doing the same for euros
// - Both should be comparable to their own type for equality
// - Both should be printable with debug format
// 
// Write a test that exercises `Usd + Usd`, `Eur + Eur`, the conversions, and equality. Put a commented-out line attempting `usd + eur` so you can verify the type system rejects it if uncommented.
// 
// A note on tuple structs: `struct Usd(u64)` defines a struct whose single field is unnamed and accessed with `.0`:
// 
// ```rust
// let amount = Usd(500);
// let cents = amount.0;       // u64
// ```
// 
// You can still implement traits on tuple structs exactly like named-field structs.

use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Usd(u64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Eur(u64);

impl Usd {
    pub fn to_dollars(&self) -> f64 {
        (self.0 as f64) / 100.0
    }
}

impl Eur {
    pub fn to_euros(&self) -> f64 {
        (self.0 as f64) / 100.0
    }   
}

impl Add for Usd {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Usd(self.0 + rhs.0)
    }
}

impl Sub for Usd {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Usd(self.0 - rhs.0)
    }
}

impl Add for Eur {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Eur(self.0 + rhs.0)
    }
}

impl Sub for Eur {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Eur(self.0 - rhs.0)
    }
}

impl From<u64> for Usd {
    fn from(value: u64) -> Self {
        Usd(value)
    } 
}

impl From<u64> for Eur {
    fn from(value: u64) -> Self {
        Eur(value)
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation() {
        let add_usd = Usd(300) + Usd(120);
        let sub_usd = Usd(523) - Usd(220);
        let add_eur = Eur(300) + Eur(120);
        let sub_eur = Eur(523) - Eur(220);
        // let invalid = Usd(100) + Eur(200);  
        // shouldn't compile
        let from_into: Usd = 500u64.into();
        let from_explicit = Eur::from(750);


        assert_eq!(from_into, Usd(500));
        assert_eq!(from_explicit, Eur(750));
        assert_eq!(add_usd, Usd(420));
        assert_eq!(sub_usd, Usd(303));
        assert_eq!(add_eur, Eur(420));
        assert_eq!(sub_eur, Eur(303));
    }

    #[test]
    fn test_conversion() {
        assert_eq!(2.0, Usd(200).to_dollars());
        assert_eq!(2.0, Eur(200).to_euros());
    }
}
