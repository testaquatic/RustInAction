use std::{net::UdpSocket, time::Duration};

use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, TimeZone, Timelike, Utc};

const NTP_MESSAGE_LENGTH: usize = 48;
const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;
const LOCAL_ADDR: &str = "0.0.0.0:12300";

struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

struct NTPMessage {
    data: [u8; NTP_MESSAGE_LENGTH],
}

impl NTPResult {
    fn delay(&self) -> i64 {
        let duration = (self.t4 - self.t1) - (self.t3 - self.t2);

        duration.num_milliseconds()
    }

    fn offset(&self) -> i64 {
        let delta = self.delay();

        delta.abs() / 2
    }
}

impl From<NTPTimestamp> for DateTime<Utc> {
    fn from(ntp: NTPTimestamp) -> Self {
        let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
        let mut nanos = ntp.fraction as f64;
        nanos *= 1e9;
        nanos /= 2.0f64.powi(32);

        Utc.timestamp_opt(secs, nanos as u32).unwrap()
    }
}

impl From<DateTime<Utc>> for NTPTimestamp {
    fn from(utc: DateTime<Utc>) -> Self {
        let secs = utc.timestamp() + NTP_TO_UNIX_SECONDS;
        let mut fraction = utc.nanosecond() as f64;
        fraction *= 2.0f64.powi(32);
        fraction /= 1e9;

        NTPTimestamp {
            seconds: secs as u32,
            fraction: fraction as u32,
        }
    }
}

impl NTPMessage {
    fn new() -> Self {
        NTPMessage {
            data: [0; NTP_MESSAGE_LENGTH],
        }
    }

    fn client() -> Self {
        const VERSION: u8 = 0b00_011_000;
        const MODE: u8 = 0b00_000_011;

        let mut msg = NTPMessage::new();
        msg.data[0] = VERSION | MODE;

        msg
    }

    fn parse_timestamp(&self, i: usize) -> Result<NTPTimestamp, std::io::Error> {
        let mut reader = &self.data[i..i + 8];
        let seconds = reader.read_u32::<BigEndian>()?;
        let fraction = reader.read_u32::<BigEndian>()?;

        Ok(NTPTimestamp { seconds, fraction })
    }

    fn rx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
        self.parse_timestamp(32)
    }

    fn tx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
        self.parse_timestamp(40)
    }
}

fn weighted_mean(values: &[f64], weights: &[f64]) -> f64 {
    let (results, sum_of_weight) = values
        .iter()
        .zip(weights)
        .fold((0.0, 0.0), |(results, sum_of_weights), (v, w)| {
            (results + v * w, sum_of_weights + w)
        });

    results / sum_of_weight
}

fn ntp_roundtrip(host: &str, port: u16) -> Result<NTPResult, std::io::Error> {
    let destination = format!("{}:{}", host, port);
    let timeout = Duration::from_secs(1);

    let reqeust = NTPMessage::client();
    let mut response = NTPMessage::new();

    let message = reqeust.data;

    let udp = UdpSocket::bind(LOCAL_ADDR)?;
    udp.connect(&destination).expect("unable to connect");

    let t1 = Utc::now();

    udp.send(&message)?;
    udp.set_read_timeout(Some(timeout))?;
    udp.recv_from(&mut response.data)?;
    let t4 = Utc::now();

    let t2 = response.rx_time().unwrap().into();
    let t3 = response.tx_time().unwrap().into();

    Ok(NTPResult { t1, t2, t3, t4 })
}

pub fn check_time() -> Result<f64, std::io::Error> {
    const NTP_PORT: u16 = 123;

    let servers = [
        "time.nist.gov",
        "time.apple.com",
        "time.cloudflare.com",
        "time.google.com",
        "time2.google.com",
    ];

    let times = servers
        .iter()
        .inspect(|server| println!("{} =>", server))
        .map(|server| ntp_roundtrip(server, NTP_PORT))
        .filter_map(|calc| match calc {
            Ok(time) => {
                println!(" {}ms away from local system time", time.offset());
                Some(time)
            }
            Err(_) => {
                println!(" ? [response took too long]");
                None
            }
        })
        .collect::<Vec<_>>();

    let mut offsets = Vec::with_capacity(times.len());
    let mut offset_weights = Vec::with_capacity(times.len());
    times.iter().for_each(|time| {
        let offset = time.offset() as f64;
        let delay = time.delay() as f64;

        let weight = 1_000_000.0 / (delay * delay);
        if weight.is_finite() {
            offsets.push(offset);
            offset_weights.push(weight);
        }
    });

    let avg_offset = weighted_mean(&offsets, &offset_weights);

    Ok(avg_offset)
}
