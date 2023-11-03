use reqwest;
use trust_dns::rr::Name;
use trust_dns::serialize::binary::{BinEncoder, BinEncodable};
use std::fmt::Display;
use std::io::prelude::*;
use std::net::{TcpStream, UdpSocket, SocketAddr, Ipv6Addr, AddrParseError};
use std::time::Duration;

use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::rr::record_type::RecordType;

use std::io;
use std::net;
use std::fs::File;
use std::error;

pub fn run() {
    let mut resp = reqwest::get("https://bt529.com/").unwrap();
    println!("resp is {}", resp.text().unwrap());

    tcp_request().unwrap();

    dns_query();

    return_error().unwrap();
}

fn tcp_request() -> std::io::Result<()> {
    let host = "www.rustinaction.com:80";
    let mut con = TcpStream::connect(host)?;
    
    con.write_all(b"GET / HTTP/1.0")?;
    con.write_all(b"\r\n")?;
    con.write_all(b"Host: www.rustinaction.com")?;
    con.write_all(b"\r\n\r\n")?;

    std::io::copy(&mut con, &mut std::io::stdout())?;

    Ok(())
}

fn dns_query() {
    let mut msg = Message::new();
    let domain_name = Name::from_ascii("www.rustinaction.com").unwrap();
    msg.set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512); //这种创建方式 长度为0 容量512
    let mut response_as_bytes: Vec<u8> = vec![0; 512];  //这种创建方式 长度512 容量512
    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    msg.emit(&mut encoder).unwrap();

    let localhost = UdpSocket::bind("0.0.0.0:0").expect("failed to bind to local socket!");
    localhost.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
    localhost.set_nonblocking(false).unwrap();
    localhost.send_to(&request_as_bytes, "180.76.76.76:53".parse::<SocketAddr>().unwrap()).unwrap();
    localhost.recv_from(&mut response_as_bytes).unwrap();

    let dns_msg = Message::from_vec(&response_as_bytes).unwrap();
    println!("{:?}", dns_msg);
    for i in dns_msg.answers() {
        if i.record_type() == RecordType::A {
            let resp = i.rdata();
            let ip = resp.to_ip_addr().unwrap();
            println!("{}", ip.to_string());
        }
    }
}


#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl Display for UpstreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError {}


impl From<io::Error> for UpstreamError {
    fn from(value: io::Error) -> Self {
        UpstreamError::IO(value)
    }
}

impl From<net::AddrParseError> for UpstreamError {
    fn from(value: net::AddrParseError) -> Self {
        UpstreamError::Parsing(value)
    }
}

fn return_error() -> Result<(), UpstreamError> {
    let f = File::open("./Cargo.toml").map_err(UpstreamError::IO);
    let localhost = "".parse::<Ipv6Addr>().map_err(UpstreamError::Parsing);
    match localhost {
        Ok(e) => println!("return ok is {}", e),
        Err(err) => eprintln!("return error is {}", err)
    }
    // let f = File::open("./Cargo.toml")?;  //使用到了前面的From trait,它将io::Error和net::AddrParseError转成了UpstreamError
    // let localhost = "".parse::<Ipv6Addr>()?;
    Ok(())
}



