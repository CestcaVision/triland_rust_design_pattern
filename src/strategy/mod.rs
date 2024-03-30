//! # Strategy Pattern
//! Strategy Pattern is a behavioral design pattern that lets you define a family of algorithms, put each of them into a separate class, and make their objects interchangeable. I implement this by fn type, a little like delegate in C#.
pub type RouteStrategy = fn(&str, &str) -> String;

pub fn walking_strategy(start: &str, end: &str) -> String {
    format!("Walking from {} to {}", start, end)
}

pub fn public_transport_strategy(start: &str, end: &str) -> String {
    format!("Taking public transport from {} to {}", start, end)
}

pub struct Navigator {
    route_strategy: RouteStrategy,
}

impl Navigator {
    pub fn new(route_strategy: RouteStrategy) -> Self {
        Navigator { route_strategy }
    }
    pub fn navigate(&self, start: &str, end: &str) -> String {
        (self.route_strategy)(start, end)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_pattern_functionality() {
        let walking_navigator = Navigator::new(walking_strategy);
        assert_eq!(walking_navigator.navigate("A", "B"), "Walking from A to B");
        let public_transport_navigator = Navigator::new(public_transport_strategy);
        assert_eq!(
            public_transport_navigator.navigate("A", "B"),
            "Taking public transport from A to B"
        );
    }
}
