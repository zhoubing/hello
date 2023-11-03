use std::fmt;
use std::fmt::Display;
use rand::RngCore;
use smoltcp::wire;

#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5])
    }
}

impl MacAddress {
    pub fn new() -> MacAddress {
        let mut data: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut data);
        MacAddress(data);
        // data[0] |= 0b_0000_0010; //这块书上应该是错误了 书上是0b_0000_0011
        //https://github.com/rust-in-action/code/issues/7
        //https://blog.csdn.net/m0_71863119/article/details/126588510
        //https://blog.51cto.com/u_13560030/6217559
        MacAddress{0: data}
    }
}

impl Into<wire::EthernetAddress> for MacAddress {
    fn into(self) -> wire::EthernetAddress {
        wire::EthernetAddress {0: self.0}
    }
}