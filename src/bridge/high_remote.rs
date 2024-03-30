use crate::bridge::device::Device;

use crate::bridge::remote::*;

pub struct HighRemote<D: Device> {
    pub device: D,
}

impl<D: Device> HasDevice<D> for HighRemote<D> {
    fn get_mut_device(&mut self) -> &mut D {
        &mut self.device
    }

    fn get_device(&self) -> &D {
        &self.device
    }
}

impl<D: Device> Remote<D> for HighRemote<D> {}

impl<D: Device> HighRemote<D> {
    pub fn new(device: D) -> Self {
        HighRemote { device }
    }
    pub fn mute(&mut self) {
        self.get_mut_device().set_volume(0);
    }
}
