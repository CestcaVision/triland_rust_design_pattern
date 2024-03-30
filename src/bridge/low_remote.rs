use crate::bridge::device::Device;

use crate::bridge::remote::*;

pub struct LowRemote<D: Device> {
    pub device: D,
}

impl<D: Device> LowRemote<D> {
    pub fn new(device: D) -> Self {
        LowRemote { device }
    }
}
impl<D: Device> HasDevice<D> for LowRemote<D> {
    fn get_mut_device(&mut self) -> &mut D {
        &mut self.device
    }

    fn get_device(&self) -> &D {
        &self.device
    }
}

impl<D: Device> Remote<D> for LowRemote<D> {}
