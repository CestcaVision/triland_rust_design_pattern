use std::collections::{HashMap, VecDeque};

use super::{mediator_trait::Mediator, plane::Plane};

#[derive(Default)]
pub struct Airport {
    planes: HashMap<String, Box<dyn Plane>>,
    planes_waiting: VecDeque<String>,
    plane_on_ground: Option<String>,
}

impl Mediator for Airport {
    fn notify_arrival(&mut self, plane: &str) -> bool {
        if self.plane_on_ground.is_none() {
            self.plane_on_ground.replace(plane.into());
            true
        } else {
            self.planes_waiting.push_back(plane.into());
            false
        }
    }

    fn notify_departure(&mut self, plane: &str) {
        if Some(plane.into()) == self.plane_on_ground {
            self.plane_on_ground = None;
            if let Some(next_plane_name) = self.planes_waiting.pop_front() {
                let mut next_plane = self.planes.remove(&next_plane_name).unwrap();
                next_plane.arrive(self);
                self.planes.insert(next_plane_name.clone(), next_plane);
                self.plane_on_ground = Some(next_plane_name);
            }
        }
    }
}

impl Airport {
    pub fn accept(&mut self, mut plane: impl Plane + 'static) {
        if self.planes.contains_key(plane.name()) {
            println!("Plane: {} already in the arrived", plane.name());
            return;
        }
        plane.arrive(self);
        self.planes.insert(plane.name().into(), Box::new(plane));
    }
    pub fn depart(&mut self, plane_name: &'static str) {
        let plane = self.planes.remove(plane_name);
        if let Some(mut plane) = plane {
            plane.depart(self);
        } else {
            println!("{} is not on the ground", plane_name);
        }
    }
}
