use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::TcpStream,
};

use protohackers::start_server;
use tracing::info;

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt().init();

    start_server(mean_handler)?;
    Ok(())
}

fn mean_handler(mut stream: TcpStream) -> Result<(), std::io::Error> {
    info!("working on stream");

    let mut buf = vec![0; 9];
    let mut records: BTreeMap<i32, i32> = BTreeMap::new();

    loop {
        if stream.read_exact(&mut buf).is_err() {
            info!("transmission finished");
            break;
        };
        info!("data read");

        match (buf[0], to_i32(&buf[1..5]), to_i32(&buf[5..])) {
            (b'I', timestamp, price) => insert(&mut records, timestamp, price),
            (b'Q', min, max) => {
                let mean = query(&records, min, max);
                stream.write_all(&mean)?;
            }
            _ => return Err(std::io::ErrorKind::Other.into()),
        }
        info!("data written");
    }

    Ok(())
}

fn to_i32(num: &[u8]) -> i32 {
    i32::from_be_bytes(num.try_into().unwrap())
}

fn query(records: &BTreeMap<i32, i32>, min: i32, max: i32) -> [u8; 4] {
    info!("querying");

    if min > max {
        return 0_i32.to_be_bytes();
    }

    let selected = records
        .range(min..=max)
        .map(|(_, v)| *v as i64)
        .collect::<Vec<_>>();

    let mean = selected
        .iter()
        .copied()
        .sum::<i64>()
        .checked_div(selected.len() as i64)
        .unwrap_or(0) as i32;

    mean.to_be_bytes()
}

fn insert(records: &mut BTreeMap<i32, i32>, timestamp: i32, price: i32) {
    info!("inserting");
    records.insert(timestamp, price);
}
