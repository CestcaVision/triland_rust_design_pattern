use crate::bridge::device::Device;

// One point of the bridge should get the reference of the other point of the bridge.
pub trait Remote<D: Device>: HasDevice<D> {
    fn toggle_power(&mut self) {
        if self.get_device().is_enabled() {
            self.get_mut_device().disable();
        } else {
            self.get_mut_device().enable();
        }
    }
    fn volume_up(&mut self) {
        let volume = self.get_device().get_volume();
        self.get_mut_device().set_volume(volume + 1);
    }
    fn volume_down(&mut self) {
        let volume = self.get_device().get_volume();
        self.get_mut_device().set_volume(volume - 1);
    }
    fn channel_up(&mut self) {
        let channel = self.get_device().get_channel();
        self.get_mut_device().set_channel(channel + 1);
    }
    fn channel_down(&mut self) {
        let channel = self.get_device().get_channel();
        self.get_mut_device().set_channel(channel - 1);
    }
}

// the value that implement this trait will get the reference of Device

pub trait HasDevice<D: Device> {
    fn get_mut_device(&mut self) -> &mut D;
    fn get_device(&self) -> &D;
}
