use std::net::{TcpListener, TcpStream};

use tracing::info;

pub fn start_server<H>(mut handler: H) -> Result<(), eyre::ErrReport>
where
    H: FnMut(TcpStream) -> Result<(), std::io::Error> + Send + Copy,
{
    let port = 7070;
    info!("binding to {port}");
    let listener = TcpListener::bind(format!(":::{port}"))?;
    info!("socket bind done");
    std::thread::scope(|s| {
        for stream in listener.incoming() {
            info!("receiving stream");
            let Ok(stream) = stream else {
                info!("connection failed");
                continue;
            };

            info!("sending to worker");
            s.spawn(move || handler(stream));
        }
    });
    Ok(())
}
