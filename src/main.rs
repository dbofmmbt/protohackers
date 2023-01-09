use std::{
    io::{ErrorKind, Read, Write},
    net::TcpListener,
};

use tracing::info;

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt().init();
    let port = 7070;
    info!("binding to {port}");
    let listener = TcpListener::bind(format!(":::{port}"))?;
    info!("socket bind done");
    std::thread::scope(|s| {
        for stream in listener.incoming() {
            info!("receiving stream");
            let Ok(mut stream) = stream else {
                info!("connection failed");
                continue;
            };

            info!("sending to worker");
            s.spawn(move || {
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
            });
        }
    });
    Ok(())
}

fn something_was_read(amount: usize) -> std::io::Result<usize> {
    if amount > 0 {
        Ok(amount)
    } else {
        Err(ErrorKind::Other.into())
    }
}
