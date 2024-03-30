use crate::bridge::device::Device;

#[derive(Default, Clone)]
pub struct Tv {
    on: bool,
    pub volume: usize,
    pub channel: usize,
}
impl Device for Tv {
    fn is_enabled(&self) -> bool {
        self.on
    }

    fn enable(&mut self) {
        self.on = true;
        println!("Radio is on");
    }

    fn disable(&mut self) {
        self.on = false;
        println!("Radio is off");
    }

    fn get_volume(&self) -> usize {
        self.volume
    }

    fn set_volume(&mut self, volume: usize) {
        self.volume = volume;
        println!("Radio volume is set to {}", volume);
    }

    fn get_channel(&self) -> usize {
        self.channel
    }

    fn set_channel(&mut self, channel: usize) {
        self.channel = channel;
        println!("Radio channel is set to {}", channel);
    }
    fn print_status(&self) {
        println!("Tv is on: {}", self.on);
        println!("Tv volume: {}", self.volume);
        println!("Tv channel: {}", self.channel);
    }
}
