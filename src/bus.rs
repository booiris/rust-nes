use std::sync::mpsc::{Receiver, Sender};

pub struct BUS {
    pub sender: Sender<(u16, u8)>,
    pub receiver: Receiver<(u16, u8)>,
}

impl BUS {
    pub fn new(sender: Sender<(u16, u8)>, receiver: Receiver<(u16, u8)>) -> Self {
        BUS { sender, receiver }
    }

    pub fn receive_data(&self, expect_addr: u16) -> u8 {
        let data = self.receiver.recv().expect("receive error data!");
        if data.0 != expect_addr {
            panic!("receive wrong addr data!")
        }
        data.1
    }

    pub fn send_data(&self, addr: u16, data: u8) {
        self.sender.send((addr, data)).expect("send error data!");
    }
}
