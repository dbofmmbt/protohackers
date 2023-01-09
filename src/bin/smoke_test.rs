use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
};

use protohackers::start_server;
use tracing::info;

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt().init();

    start_server(echo_handler)?;
    Ok(())
}

fn echo_handler(mut stream: TcpStream) -> Result<(), std::io::Error> {
    info!("working on stream");
    let mut buf = vec![0; 1024];
    loop {
        let Ok(amount) = stream.read(&mut buf).and_then(something_was_read) else {
            info!("transmission finished");
            break;
        };

        info!("data read");
        stream.write_all(&buf[..amount])?;
        info!("data written");
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
