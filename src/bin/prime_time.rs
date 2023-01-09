use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
    time::Duration,
};

use protohackers::start_server;
use tracing::{info, warn};

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt().init();

    start_server(prime_handler)?;
    Ok(())
}

fn prime_handler(mut stream: TcpStream) -> Result<(), std::io::Error> {
    info!("working on stream");
    let mut buf = vec![0; 1024];

    let mut line = String::new();

    stream.set_read_timeout(Some(Duration::from_secs(1)))?;

    loop {
        let Ok(amount) = stream.read(&mut buf).and_then(something_was_read) else {
            info!("transmission finished");
            break;
        };

        let input = std::str::from_utf8(&buf[..amount]).unwrap();
        info!("input was {input}");
        let lines = input.split('\n').collect::<Vec<_>>();
        for json in &lines[..lines.len() - 1] {
            info!("processing {json}");
            line.push_str(json);
            process_request(&mut stream, &line)?;

            line.clear();
        }
        if let Some(end) = lines.last() {
            line.push_str(end);
        }
    }
    Ok::<_, std::io::Error>(())
}

fn something_was_read(amount: usize) -> std::io::Result<usize> {
    if amount > 0 {
        Ok(amount)
    } else {
        Err(ErrorKind::Other.into())
    }
}

macro_rules! handle_failure {
    ($stream:ident) => {
        send_malformed_response($stream)?;
        return Err(std::io::ErrorKind::Other.into());
    };
}

fn send_malformed_response(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    stream.write_all(b"{}\n")?;
    Ok(())
}

fn process_request(stream: &mut TcpStream, line: &str) -> Result<(), std::io::Error> {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(line) else {
        warn!("didn't parse as json");
        handle_failure!(stream);
    };

    let (Some(method), Some(number)) = (json.get("method"), json.get("number")) else {
        warn!("field missing");
        handle_failure!(stream);
    };

    let Some("isPrime") = method.as_str() else {
        warn!("wrong method");
        handle_failure!(stream);
    };

    if !number.is_number() {
        warn!("number field doesn't contain a number");
        handle_failure!(stream);
    }

    if number.is_f64() {
        send_answer(stream, false)?;
        return Ok(());
    }

    let Some(number) = number.as_i64() else {
        handle_failure!(stream);
    };

    send_answer(stream, is_prime(number))?;
    Ok(())
}

fn send_answer(mut stream: &mut TcpStream, prime: bool) -> Result<(), std::io::Error> {
    serde_json::to_writer(
        &mut stream,
        &serde_json::json!({"method": "isPrime", "prime": prime}),
    )?;
    stream.write_all(&[b'\n'])?;
    Ok(())
}

fn is_prime(num: i64) -> bool {
    primes::is_prime(num as u64)
}
