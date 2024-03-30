//! #Bridge Pattern
//! Bridge Pattern is a structural design pattern that lets you split a large class or a set of closely related classes into two separate hierarchies—abstraction and implementation—which can be developed independently of each other.
pub mod device;
pub mod high_remote;
pub mod low_remote;
pub mod radio;
pub mod remote;
pub mod tv;

pub use device::*;
pub use high_remote::*;
pub use low_remote::*;
pub use radio::*;
pub use remote::*;
pub use tv::*;

#[cfg(test)]
mod tests {

    use super::*;
    fn test_device(device: impl Device) {
        let mut low_remote = LowRemote::new(device.clone());
        println!("=====LOW REMOTE=====");
        low_remote.device.print_status();
        low_remote.toggle_power();
        low_remote.volume_up();
        low_remote.channel_up();
        low_remote.device.print_status();

        let mut high_remote = HighRemote::new(device.clone());
        println!("=====HIGH REMOTE=====");
        high_remote.device.print_status();
        high_remote.toggle_power();
        high_remote.volume_up();
        high_remote.channel_up();
        high_remote.mute();
        high_remote.device.print_status();
    }

    #[test]
    fn test_bridge_pattern_functionality() {
        let tv = Tv::default();
        test_device(tv);
        let radio = Radio::default();
        test_device(radio);
    }
}
