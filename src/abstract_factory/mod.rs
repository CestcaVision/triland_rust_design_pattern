//! # Abstract Factory Pattern
//! Abstract Factory Pattern is a creational design pattern that provides an interface for creating families of related or dependent objects without specifying their concrete classes， and maintains the decoupling and extensibility of the code at the same time.

// define a trait for general pizza, including necessary methods for pizza
pub trait Pizza {
    fn prepare(&self) -> String;
    fn bake(&self) -> String;
    fn cut(&self) -> String;
    fn box_it(&self) -> String;
}

// define different types of pizza, including cheese, pepperoni, in different culture: Italian and American
pub struct ItalianCheesePizza;
pub struct ItalianPepperoniPizza;
pub struct AmericanCheesePizza;
pub struct AmericanPepperoniPizza;

impl Pizza for ItalianCheesePizza {
    fn prepare(&self) -> String {
        "Preparing Italian Cheese Pizza".to_string()
    }

    fn bake(&self) -> String {
        "Baking Italian Cheese Pizza".to_string()
    }

    fn cut(&self) -> String {
        "Cutting Italian Cheese Pizza".to_string()
    }

    fn box_it(&self) -> String {
        "Boxing Italian Cheese Pizza".to_string()
    }
}
impl Pizza for ItalianPepperoniPizza {
    fn prepare(&self) -> String {
        "Preparing Italian Pepperoni Pizza".to_string()
    }

    fn bake(&self) -> String {
        "Baking Italian Pepperoni Pizza".to_string()
    }

    fn cut(&self) -> String {
        "Cutting Italian Pepperoni Pizza".to_string()
    }

    fn box_it(&self) -> String {
        "Boxing Italian Pepperoni Pizza".to_string()
    }
}
impl Pizza for AmericanCheesePizza {
    fn prepare(&self) -> String {
        "Preparing American Cheese Pizza".to_string()
    }

    fn bake(&self) -> String {
        "Baking American Cheese Pizza".to_string()
    }

    fn cut(&self) -> String {
        "Cutting American Cheese Pizza".to_string()
    }

    fn box_it(&self) -> String {
        "Boxing American Cheese Pizza".to_string()
    }
}
impl Pizza for AmericanPepperoniPizza {
    fn prepare(&self) -> String {
        "Preparing American Pepperoni Pizza".to_string()
    }

    fn bake(&self) -> String {
        "Baking American Pepperoni Pizza".to_string()
    }

    fn cut(&self) -> String {
        "Cutting American Pepperoni Pizza".to_string()
    }

    fn box_it(&self) -> String {
        "Boxing American Pepperoni Pizza".to_string()
    }
}
// define a trait for general pizza factory, with a method to create a pizza, with parameter of pizza type: cheese, pepperoni, or veggie

pub trait PizzaFactory {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza>;
}

// implement Two Factories for different culture of pizza: Italian and American
pub struct ItalianPizzaFactory;
pub struct AmricanPizzaFactory;

impl PizzaFactory for ItalianPizzaFactory {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        match pizza_type {
            "cheese" => Box::new(ItalianCheesePizza),
            "pepperoni" => Box::new(ItalianPepperoniPizza),
            _ => panic!("Pizza type not found"),
        }
    }
}

impl PizzaFactory for AmricanPizzaFactory {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        match pizza_type {
            "cheese" => Box::new(AmericanCheesePizza),
            "pepperoni" => Box::new(AmericanPepperoniPizza),
            _ => panic!("Pizza type not found"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pizza_production() {
        let factory = ItalianPizzaFactory;
        assert!(factory.create_pizza("cheese").prepare() == "Preparing Italian Cheese Pizza");
        let factory2 = AmricanPizzaFactory;
        assert!(factory2.create_pizza("pepperoni").box_it() == "Boxing American Pepperoni Pizza");
    }
}
