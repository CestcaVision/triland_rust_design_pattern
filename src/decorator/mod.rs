//! # Decorator Pattern
//! Decorator pattern is a structural design pattern that allows adding new behaviors to objects dynamically by placing them inside special wrapper objects that contain these behaviors.

pub trait Coffee {
    fn ingredients(&self) -> String;
}

pub struct BlackCoffee;

impl Coffee for BlackCoffee {
    fn ingredients(&self) -> String {
        "Black Coffee".to_string()
    }
}

// Decorator struct for adding milk to coffee, which including a pointer to the coffee object.

pub struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl MilkDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        MilkDecorator { coffee }
    }
}

impl Coffee for MilkDecorator {
    fn ingredients(&self) -> String {
        format!("{}, Milk", self.coffee.ingredients())
    }
}

pub struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}

impl SugarDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        SugarDecorator { coffee }
    }
}
impl Coffee for SugarDecorator {
    fn ingredients(&self) -> String {
        format!("{}, Sugar", self.coffee.ingredients())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decorator_pattern() {
        let simple_coffee = BlackCoffee;
        assert_eq!(simple_coffee.ingredients(), "Black Coffee");
        let milk_coffee = MilkDecorator::new(Box::new(simple_coffee));
        assert_eq!(milk_coffee.ingredients(), "Black Coffee, Milk");
        let milk_and_sugar_coffee = SugarDecorator::new(Box::new(milk_coffee));
        assert_eq!(
            milk_and_sugar_coffee.ingredients(),
            "Black Coffee, Milk, Sugar"
        );
    }
}
