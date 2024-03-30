pub trait Mediator {
    fn notify_arrival(&mut self, plane: &str) -> bool;
    fn notify_departure(&mut self, plane: &str);
}
