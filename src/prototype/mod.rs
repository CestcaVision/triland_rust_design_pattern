//! #Prototype Pattern
//! Prototype pattern is a creational design pattern that allows cloning objects, even complex ones, without coupling to their specific classes.

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Position { x, y, z }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prototype_pattern_functionality() {
        let position = Position::new(1.0, 2.0, 3.0);
        let cloned_position = position.clone();
        assert_eq!(position.x, cloned_position.x);
        assert_eq!(position.y, cloned_position.y);
        assert_eq!(position.z, cloned_position.z);
        assert_eq!(position, cloned_position);
    }
}
