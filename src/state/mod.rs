//! # State Pattern
//! State pattern is a behavioral design pattern that allows an object to alter its behavior when its internal state changes. The object will appear to change its class.

pub enum Action {
    StartWalking,
    StartRunning,
    Stop,
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Idle,
    Walking,
    Running,
}

impl State {
    fn update(&mut self, action: Action) {
        *self = match (*self, action) {
            (_, Action::StartWalking) => State::Walking,
            (_, Action::StartRunning) => State::Running,
            (_, Action::Stop) => State::Idle,
            _ => return,
        };
    }
}
struct AnimationStateMachine {
    current_state: State,
}

impl AnimationStateMachine {
    // 创建新的状态机实例
    fn new(initial_state: State) -> Self {
        AnimationStateMachine {
            current_state: initial_state,
        }
    }

    // 根据动作更新状态机的状态, 注意这里也使用 &mut self
    fn update(&mut self, action: Action) {
        self.current_state.update(action);
        println!("Current State: {:?}", self.current_state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_pattern_functionality() {
        let initial_state = State::Idle;
        let mut animation_state_machine = AnimationStateMachine::new(initial_state);

        // 模拟一系列动作来更新状态机的状态
        animation_state_machine.update(Action::StartWalking);
        // 应输出：Current State: Walking
        animation_state_machine.update(Action::StartRunning);
        // 应输出：Current State: Running
        animation_state_machine.update(Action::Stop);
        // 应输出：Current State: Idle
    }
}
