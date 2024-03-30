//! Observer pattern
//! Observer pattern is a behavioral design pattern that defines a one-to-many dependency between objects so that when one object changes state, all its dependents are notified and updated automatically.
//! In this example, I implement a newsletter system that allows subscribers to subscribe to a newsletter and receive notifications when a new message is sent.

pub trait Subscriber {
    fn update(&mut self, message: &str);
}

pub trait Newsletter<'a> {
    fn add_subscriber(&mut self, subscriber: &'a mut dyn Subscriber);
    fn remove_subscriber(&mut self, index: usize);
    fn notify_subscribers(&mut self);
}

pub struct EmailSubscriber {
    pub news: String,
    pub email: String,
}

impl EmailSubscriber {
    fn new(email: &str) -> EmailSubscriber {
        EmailSubscriber {
            email: email.to_string(),
            news: String::new(),
        }
    }
}

impl Subscriber for EmailSubscriber {
    fn update(&mut self, message: &str) {
        self.news = format!("User: {}\nGot a message:{}", self.email, message);
    }
}

pub struct EmailNewsletter<'a> {
    subscribers: Vec<&'a mut dyn Subscriber>,
    message: String,
}

impl<'a> EmailNewsletter<'a> {
    fn new() -> EmailNewsletter<'a> {
        EmailNewsletter {
            subscribers: Vec::new(),
            message: String::new(),
        }
    }

    fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }
}

impl<'a> Newsletter<'a> for EmailNewsletter<'a> {
    fn add_subscriber(&mut self, subscriber: &'a mut dyn Subscriber) {
        self.subscribers.push(subscriber);
    }

    fn remove_subscriber(&mut self, index: usize) {
        if index < self.subscribers.len() {
            self.subscribers.remove(index);
        }
    }

    fn notify_subscribers(&mut self) {
        for sub in &mut self.subscribers {
            sub.update(&self.message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer_pattern_functionality() {
        let mut newsletter = EmailNewsletter::new();

        let mut subscriber1 = EmailSubscriber::new("user1@example.com");
        let mut subscriber2 = EmailSubscriber::new("user2@example.com");
        newsletter.add_subscriber(&mut subscriber1);
        newsletter.add_subscriber(&mut subscriber2);

        newsletter.set_message("Welcome to our newsletter!");
        newsletter.notify_subscribers();
        assert_eq!(
            subscriber1.news,
            "User: user1@example.com\nGot a message:Welcome to our newsletter!"
        );
        assert_eq!(
            subscriber2.news,
            "User: user2@example.com\nGot a message:Welcome to our newsletter!"
        );
    }
}
