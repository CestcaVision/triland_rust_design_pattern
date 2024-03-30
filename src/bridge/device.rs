pub trait Device: Clone {
    fn is_enabled(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);
    fn get_volume(&self) -> usize;
    fn set_volume(&mut self, volume: usize);
    fn get_channel(&self) -> usize;
    fn set_channel(&mut self, channel: usize);
    fn print_status(&self);
}
