use rand::RngCore;

#[derive(Debug)]
struct MacAddress([u8; 6]);    

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

    pub fn is_local(&self) -> bool {
        (self.0[0] & 0b_0000_0010) == 0b_0000_0010
    }

    pub fn is_unicast(&self) -> bool {
        (self.0[0] & 0b_0000_0010) == 0b_0000_0010
    }
}

pub fn run() {
    let mac = MacAddress::new();
    println!("{:?}", mac);

    run_state_machine();
}

enum HttpState {
    Connect,
    Request,
    Response
}

pub fn run_state_machine() {
    // loop {
    //     match state {
    //         HttpState::Connect
    //     }
    // }
}