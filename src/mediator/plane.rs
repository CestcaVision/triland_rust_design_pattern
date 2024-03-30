use crate::mediator::mediator_trait::Mediator;
pub trait Plane {
    fn name(&self) -> &str;
    fn arrive(&mut self, mediator: &mut dyn Mediator);
    fn depart(&mut self, mediator: &mut dyn Mediator);
}

pub struct CargoPlane {
    name: String,
}

impl CargoPlane {
    pub fn new(name: &str) -> CargoPlane {
        CargoPlane {
            name: name.to_string(),
        }
    }
}
pub struct PassengerPlane {
    name: String,
}
impl PassengerPlane {
    pub fn new(name: &str) -> PassengerPlane {
        PassengerPlane {
            name: name.to_string(),
        }
    }
}

impl Plane for CargoPlane {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_arrival(&self.name()) {
            println!("Cargo Plane: {}, Arrival Blocked, Waiting", self.name());
        } else {
            println!("Cargo Plane: {}, Landing", self.name());
        }
    }

    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("Cargo Plane: {}, Leaving", self.name());
        mediator.notify_departure(&self.name());
    }
}

impl Plane for PassengerPlane {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_arrival(&self.name()) {
            println!("Passenger Plane: {}, Arrival Blocked, Waiting", self.name());
        } else {
            println!("Passenger Plane: {}, Landing", self.name());
        }
    }

    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("Passenger Plane: {}, Leaving", self.name());
        mediator.notify_departure(&self.name());
    }
}
