//! # Command Pattern
//! Command pattern is a behavioral design pattern that turns a request into a stand-alone object that contains all information about the request. This transformation lets you pass requests as a method arguments, delay or queue a request’s execution, and support undoable operations.
pub trait Command {
    fn execute(&self);
}

pub struct Receiver;

impl Receiver {
    fn action(&self) {
        println!("Receiver: List all files");
    }
}

struct CommandImpl {
    receiver: Receiver,
}

impl CommandImpl {
    fn new(receiver: Receiver) -> Self {
        CommandImpl { receiver }
    }
}

impl Command for CommandImpl {
    fn execute(&self) {
        println!("Command Send to Recevier");
        self.receiver.action();
    }
}

struct Invoker {
    command: Box<dyn Command>,
}

impl Invoker {
    fn new(command: Box<dyn Command>) -> Self {
        Invoker { command }
    }

    fn call(&self) {
        println!("Invoker: call command ls");
        self.command.execute();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_pattern_functionality() {
        let receiver = Receiver;

        let command = CommandImpl::new(receiver);

        let invoker = Invoker::new(Box::new(command));

        invoker.call();
    }
}
