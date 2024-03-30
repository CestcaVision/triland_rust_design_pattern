//! # Adapter Pattern
//! Adapter pattern is a structural design pattern that allows objects with incompatible interfaces to collaborate.

pub trait RoundHole {
    fn get_radius(&self) -> f64;
    fn fits(&self, peg: &dyn RoundPeg) -> bool;
}

//  regard adapter target obj as a trait
pub trait RoundPeg {
    fn get_radius(&self) -> f64;
}

// also regard adapter source as a trait
pub trait SquarePeg {
    fn get_width(&self) -> f64;
}

// implement target interface for obj which implements source interface
impl RoundPeg for &dyn SquarePeg {
    fn get_radius(&self) -> f64 {
        self.get_width() * 2.0_f64.sqrt() / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct RoundHoleImpl {
        radius: f64,
    }
    impl RoundHole for RoundHoleImpl {
        fn get_radius(&self) -> f64 {
            self.radius
        }
        fn fits(&self, peg: &dyn RoundPeg) -> bool {
            peg.get_radius() <= self.get_radius()
        }
    }
    struct RoundPegImpl {
        radius: f64,
    }
    impl RoundPeg for RoundPegImpl {
        fn get_radius(&self) -> f64 {
            self.radius
        }
    }
    struct SquarePegImpl {
        width: f64,
    }
    impl SquarePeg for SquarePegImpl {
        fn get_width(&self) -> f64 {
            self.width
        }
    }

    #[test]
    fn test_adapter_pattern_functionality() {
        let round_hole = RoundHoleImpl { radius: 2.0 };
        let round_peg = RoundPegImpl { radius: 2.0 };
        assert!(round_hole.fits(&round_peg));
        let square_peg = SquarePegImpl { width: 2.828 };
        // transform square_peg to abstract SquarePeg explicitly
        // then it can be used as RoundPeg implicitly
        assert!(round_hole.fits(&(&square_peg as &dyn SquarePeg)));
    }
}
