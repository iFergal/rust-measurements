//! Types and constants for handling electrical charge.

use super::measurement::*;

/// Magnitude of speed of light (m/s)
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// The `Charge` struct can be used to deal with electrical charge in a
/// common way.
///
/// # Example
///
/// ```
/// use measurements::Charge;
///
/// let ch = Charge::from_coulombs(72.0);
/// let c = ch.as_coulombs();
/// let ab_c = ch.as_abcoulombs();
/// println!("A charge of {} C has {} abC", c, ab_c);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Charge {
    coulombs: f64,
}

impl Charge {
    /// Create a new Charge from a floating point value in coulombs
    pub fn from_coulombs(coulombs: f64) -> Self {
        Charge { coulombs }
    }

    /// Create a new Charge from a floating point value in abcoulombs
    pub fn from_abcoulombs(abcoulombs: f64) -> Self {
        Self::from_coulombs(abcoulombs * 10.0)
    }

    /// Create a new Charge from a floating point value in coulombs
    pub fn from_statcoulombs(statcoulombs: f64) -> Self {
        Self::from_coulombs(statcoulombs / (10.0 * SPEED_OF_LIGHT))
    }

    /// Convert this Charge into a floating point value in statcoulombs
    pub fn as_coulombs(&self) -> f64 {
        self.coulombs
    }

    /// Convert this Charge into a floating point value in abcoulombs
    pub fn as_abcoulombs(&self) -> f64 {
        self.coulombs / 10.0
    }

    /// Convert this Charge into a floating point value in statcoulombs
    pub fn as_statcoulombs(&self) -> f64 {
        self.coulombs * (10.0 * SPEED_OF_LIGHT)
    }
}

impl Measurement for Charge {
    fn as_base_units(&self) -> f64 {
        self.coulombs
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_coulombs(units)
    }

    fn get_base_units_name(&self) -> &'static str {
        "C"
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        // Smallest to Largest
        let list = [
            ("fC", 1e-15),
            ("pC", 1e-12),
            ("nC", 1e-9),
            ("\u{00B5}C", 1e-6),
            ("mC", 1e-3),
            ("C", 1e0),
            ("kC", 1e3),
            ("MC", 1e6),
            ("GC", 1e9),
            ("TC", 1e12),
            ("PC", 1e15),
            ("EC", 1e18),
        ];
        self.pick_appropriate_units(&list)
    }
}

implement_measurement! { Charge }

#[cfg(test)]
mod test {
    use charge::*;
    use test_utils::assert_almost_eq;

    #[test]
    pub fn as_coulombs() {
        let u = Charge::from_abcoulombs(1.234);
        assert_almost_eq(u.as_coulombs(), 12.340);
    }

    #[test]
    pub fn as_abcoulombs() {
        let u = Charge::from_coulombs(10_000.0);
        assert_almost_eq(u.as_abcoulombs(), 1000.0);
    }

    #[test]
    pub fn as_statcoulombs() {
        let u = Charge::from_coulombs(0.0002);
        assert_almost_eq(u.as_statcoulombs(), 599584.916);
    }

    // Traits
    #[test]
    fn add() {
        let a = Charge::from_coulombs(20.0);
        let b = Charge::from_abcoulombs(4.0);
        let c = a + b;
        assert_almost_eq(c.as_coulombs(), 60.0);
    }

    #[test]
    fn sub() {
        let a = Charge::from_abcoulombs(20.0);
        let b = Charge::from_coulombs(150.0);
        let c = a - b;
        assert_almost_eq(c.as_coulombs(), 50.0);
    }

    #[test]
    fn mul() {
        let a = Charge::from_coulombs(10.0);
        let b = 4.0 * a;
        assert_almost_eq(b.as_abcoulombs(), 4.0);
    }

    #[test]
    fn div() {
        let a = Charge::from_abcoulombs(2.0);
        let b = Charge::from_coulombs(40.0);
        let c = a / b;
        let d = a / 2.0;
        assert_almost_eq(c, 0.5);
        assert_almost_eq(d.as_coulombs(), 10.0);
    }

    #[test]
    fn eq() {
        let a = Charge::from_coulombs(10.0);
        let b = Charge::from_abcoulombs(1.0);
        assert_eq!(a, b);
    }

    #[test]
    fn neq() {
        let a = Charge::from_coulombs(2.0);
        let b = Charge::from_abcoulombs(2.0);
        assert_ne!(a, b);
    }

    #[test]
    fn cmp() {
        let a = Charge::from_coulombs(2.0);
        let b = Charge::from_coulombs(4.0);
        assert_eq!(a < b, true);
        assert_eq!(a <= b, true);
        assert_eq!(a > b, false);
        assert_eq!(a >= b, false);
    }
}
