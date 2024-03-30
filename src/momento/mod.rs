//! # Momento Pattern
//! Memento pattern is a behavioral design pattern that lets you save and restore the previous state of an object without revealing the details of its implementation.

// Memento trait 定义了存储和恢复状态的方法
trait Memento {
    fn get_saved_state(&self) -> String;
}

// Originator 持有当前状态
struct Originator {
    state: String,
}

impl Originator {
    fn new(state: &str) -> Self {
        Originator {
            state: state.to_string(),
        }
    }

    fn set_state(&mut self, state: &str) {
        self.state = state.to_string();
    }

    fn save_state_to_memento(&self) -> Box<dyn Memento> {
        Box::new(ConcreteMemento::new(&self.state))
    }

    fn restore_state_from_memento(&mut self, memento: &Box<dyn Memento>) {
        let state = memento.get_saved_state();
        self.set_state(&state);
    }
}

// ConcreteMemento 保存 Originator 的状态
struct ConcreteMemento {
    state: String,
}

impl ConcreteMemento {
    fn new(state: &str) -> Self {
        ConcreteMemento {
            state: state.to_string(),
        }
    }
}

impl Memento for ConcreteMemento {
    fn get_saved_state(&self) -> String {
        self.state.clone()
    }
}

// Caretaker 对象负责从 Memento 中恢复对象的状态
struct Caretaker {
    saved_states: Vec<Box<dyn Memento>>,
}

impl Caretaker {
    fn new() -> Self {
        Caretaker {
            saved_states: Vec::new(),
        }
    }

    fn add_memento(&mut self, memento: Box<dyn Memento>) {
        self.saved_states.push(memento);
    }

    fn get_memento(&mut self, index: usize) -> Option<&Box<dyn Memento>> {
        self.saved_states.get(index)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memento_pattern_functionality() {
        let mut originator = Originator::new("Initial State");
        let mut caretaker = Caretaker::new();

        // 修改状态并保存到 Memento
        originator.set_state("State #1");
        caretaker.add_memento(originator.save_state_to_memento());

        originator.set_state("State #2");
        caretaker.add_memento(originator.save_state_to_memento());

        // 验证是否正确保存了第二个状态
        if let Some(memento) = caretaker.get_memento(1) {
            let state = memento.get_saved_state();
            assert_eq!(
                state, "State #2",
                "The second saved state should be 'State #2'."
            );
        } else {
            panic!("Expected a saved state at index 1, but found none.");
        }

        // 恢复到第一个保存的状态
        if let Some(memento) = caretaker.get_memento(0) {
            originator.restore_state_from_memento(memento);
            assert_eq!(
                originator.state, "State #1",
                "After restoration, the state should be 'State #1'."
            );
        } else {
            panic!("Expected a saved state at index 0, but found none.");
        }

        // 验证当前状态是否为 "State #1"
        assert_eq!(
            originator.state, "State #1",
            "The current state of Originator should be 'State #1' after restoration."
        );
    }
}
