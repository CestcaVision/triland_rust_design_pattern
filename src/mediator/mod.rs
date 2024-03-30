pub mod airport;
pub mod mediator_trait;
pub mod plane;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mediator_pattern_funtionality() {
        let plane1 = plane::CargoPlane::new("Cargo");
        let plane2 = plane::PassengerPlane::new("Airbus");
        let mut airport = airport::Airport::default();
        airport.accept(plane1);
        airport.accept(plane2);
        airport.depart("Cargo");
        airport.depart("Airbus");
        airport.depart("Bowing");
    }
}
