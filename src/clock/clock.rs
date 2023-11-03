use std::{mem::zeroed, time::Duration, net::UdpSocket};

use byteorder::{ReadBytesExt, BigEndian};
use chrono::{DateTime, TimeZone, Timelike, Duration as ChronoDuration };
use clap::{App, Arg};

struct Clock;   //不包含任何字段的结构体，叫做零大小类型。在生成的程序中不占用任何空间。

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::{timeval, time_t, suseconds_t}; //导入代码放到对应函数中，避免污染全局作用域
        use libc::{settimeofday, timezone};
        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe {
            zeroed()
        };
        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}

pub fn run() {
    let app = App::new("clock").version("0.1")
        .about("Gets")
        .arg(Arg::with_name("action").takes_value(true).possible_values(&["get", "set"]).default_value("get"))
        .arg(Arg::with_name("std").short("s").long("standard").takes_value(true).possible_values(&["rfc2822", "rfc3339", "timestamp"]).default_value("rfc3339"))
        .arg(Arg::with_name("datetime").help("When <action> is 'set', apply <datetime>. Otherwise, ignore"));
    let args = app.get_matches();

    let action = args.value_of("action").unwrap();
    let std = args.value_of("std").unwrap();
    if action == "set" {
        let datetime: &str = args.value_of("datetime").unwrap();

        let parser = match std {
            "rfc2822" => {
                DateTime::parse_from_rfc2822
            }
            "rfc3339" => {
                DateTime::parse_from_rfc3339
            }
            _ => {
                unreachable!()
            }
        };
        let msg = format!("Unable to parse {} according to {}", datetime, std);
        println!("action {}", action);
        println!("std {}", std);
        println!("datetime {}", datetime);
        let t = parser(datetime).expect(&msg);
        Clock::set(t);
    } else if action == "check-ntp" {
        let offset = check_time().unwrap() as isize;
        let adjust_ms_ = offset.signum() * offset.abs().min(200) / 5;
        let adjust_ms = ChronoDuration::microseconds(adjust_ms_ as i64);
        let now = Utc::now() + adjust_ms;
        Clock::set(now);
    }

    let maybe_error = std::io::Error::last_os_error();
    let os_error_code = maybe_error.raw_os_error();
    match os_error_code {
        Some(0) => {}
        Some(_) => { eprintln!("unable to set the time! {}", maybe_error) }
        _ => {}
    }

    let now = Clock::get();

    match std {
        "timestmp" => {
            println!("{}", now.timestamp())
        }
        "rfc2822" => {
            println!("{}", now.to_rfc2822())
        }
        "rfc3339" => {
            println!("{}", now.to_rfc3339())
        }
        _ => {
            unreachable!()
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

use chrono::{Local, Utc};

const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;

impl From<NTPTimestamp> for DateTime<Utc> {
    fn from(ntp: NTPTimestamp) -> Self {
        let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
        let mut nanos = ntp.fraction as f64;
        nanos *= 1e9;
        Utc.timestamp_opt(secs, nanos as u32).unwrap()
    }
}

impl From<DateTime<Utc>> for NTPTimestamp {
    fn from(utc: DateTime<Utc>) -> Self {
        let secs = utc.timestamp() + NTP_TO_UNIX_SECONDS;
        let mut fraction = utc.nanosecond() as f64;
        fraction *= 2_f64.powi(32);
        fraction /= 1e9;
        NTPTimestamp { seconds: secs as u32, fraction: fraction as u32 }
    }
}

const NTP_MESSAGE_LENGTH: usize = 48;
const LOCAL_ADDR: &'static str = "0.0.0.0:12300";

struct NTPMessage {
    data: [u8; NTP_MESSAGE_LENGTH],
}

struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

impl NTPResult {
    fn offset(&self) -> i64 {
        let duration = (self.t2 - self.t1) + (self.t4 - self.t3);
        duration.num_milliseconds() / 2
    }

    fn delay(&self) -> i64 {
        let duration = (self.t4 - self.t1) + (self.t3 - self.t2);
        duration.num_milliseconds()
    }
}

impl NTPMessage {
    fn new() -> Self {
        NTPMessage { data: [0; NTP_MESSAGE_LENGTH] }
    }

    fn client() -> Self {
        const VERSION: u8 = 0b00_011_000;
        const MODE: u8 = 0b00_011_011;

        let mut msg = NTPMessage::new();
        msg.data[0] |= VERSION;
        msg.data[0] |= MODE;
        msg
    }

    fn parse_timestamp(&self, i: usize) -> Result<NTPTimestamp, std::io::Error> {
        let mut reader = &self.data[i..i + 8];
        let seconds = reader.read_u32::<BigEndian>()?;
        let fraction = reader.read_u32::<BigEndian>()?;

        Ok(NTPTimestamp { seconds: seconds, fraction: fraction })
    }

    fn rx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
        self.parse_timestamp(32)
    }

    fn tx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
        self.parse_timestamp(40)
    }
}

fn weighted_mean(values: &[f64], weights: &[f64]) -> f64 {
    let mut result = 0.0;
    let mut sum_of_weights = 0.0;
    for (v, w) in values.iter().zip(weights) {
        result += v * w;
        sum_of_weights += w;
    }
    result / sum_of_weights
}

fn ntp_roundtrip(host: &str, port: u16) -> Result<NTPResult, std::io::Error> {
    let destination = format!("{}:{}", host, port);
    let timeout = Duration::from_secs(1);
    let request = NTPMessage::client();
    let mut response = NTPMessage::new();
    let message = request.data;
    let udp = UdpSocket::bind(LOCAL_ADDR)?;
    udp.connect(&destination).expect("unable to connect");
    let t1 = Utc::now();
    udp.send(&message);
    udp.set_read_timeout(Some(timeout));
    udp.recv_from(&mut response.data);
    let t4 = Utc::now();
    let t2 = response.rx_time().unwrap().into();
    let t3 = response.tx_time().unwrap().into();
    Ok(NTPResult { t1: t1, t2: t2, t3: t3, t4: t4 })
}

fn check_time() -> Result<f64, std::io::Error> {
    const NTP_PORT: u16 = 123;
    let servers = [
        "time.nist.com",
        "time.apple.com",
        "time.euro.apple.com",
        "time.google.com",
        "time2.google.com"
    ];

    let mut times = Vec::with_capacity(servers.len());
    for &server in servers.iter() {
        println!("{} => ", server);
        let calc = ntp_roundtrip(server, NTP_PORT);
        match calc {
            Ok(time) => {
                println!("{}ms away from local system time", time.offset());
                times.push(time);
            }
            Err(_) => {
                eprintln!("? [response took too long]")
            }
        }
    }

    let mut offsets = Vec::with_capacity(servers.len());
    let mut offsets_weights = Vec::with_capacity(servers.len());

    for time in &times {
        let offset = time.offset() as f64;
        let delay = time.delay() as f64;
        let weight = 1_000_000.0 / (delay * delay);
        if weight.is_finite() {
            offsets.push(offset);
            offsets_weights.push(weight);
        }
    }
    let avg_offset = weighted_mean(&offsets, &offsets_weights);
    Ok(avg_offset)
}

fn run_ntp() {}