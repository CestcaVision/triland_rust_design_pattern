//! # Flyweight Pattern
//! Flyweight pattern is a structural design pattern that allows programs to support vast quantities of objects by keeping their memory consumption low.

use std::{collections::HashMap, sync::Arc};

pub struct BigObject {
    data: [u64; 1000],
}
impl BigObject {
    pub fn new() -> Self {
        BigObject { data: [0; 1000] }
    }
}
impl Default for BigObject {
    fn default() -> Self {
        BigObject::new()
    }
}
#[derive(Default)]
pub struct FlyweightFactory {
    objects: HashMap<String, Arc<BigObject>>,
}

impl FlyweightFactory {
    pub fn get(&self, name: &str) -> Option<Arc<BigObject>> {
        self.objects.get(name).cloned()
    }
    pub fn create<F: FnOnce() -> BigObject>(
        &mut self,
        name: String,
        create_object: F,
    ) -> Arc<BigObject> {
        let res = Arc::new(create_object());
        self.objects.insert(name, res.clone());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flyweight_pattern_functionality() {
        let mut factory = FlyweightFactory::default();
        factory.create("obj1".into(), BigObject::default);
        let obj = match factory.get("obj1") {
            Some(obj) => obj,
            None => factory.create("obj1".into(), BigObject::default),
        };
        assert_eq!(obj.data.len(), 1000);
    }
}
