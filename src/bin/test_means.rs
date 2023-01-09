use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("2a09:8280:1::1:c552:7070")?;

    insert(&mut stream, 1, 5)?;
    insert(&mut stream, 2, 10)?;
    insert(&mut stream, 3, 15)?;

    let one_to_two = query(&mut stream, 1, 2)?;
    dbg!(one_to_two);

    let two_to_three = query(&mut stream, 2, 3)?;
    dbg!(two_to_three);

    let one_to_three = query(&mut stream, 1, 3)?;
    dbg!(one_to_three);

    Ok(())
}

fn query(stream: &mut TcpStream, min: i32, max: i32) -> io::Result<i32> {
    let mut input = vec![b'Q'];
    input.extend_from_slice(&min.to_be_bytes());
    input.extend_from_slice(&max.to_be_bytes());
    stream.write_all(&input)?;

    let mut number = [0; 4];
    stream.read_exact(&mut number)?;
    Ok(i32::from_be_bytes(number))
}

fn insert(stream: &mut TcpStream, timestamp: i32, price: i32) -> io::Result<()> {
    let mut input = vec![b'I'];
    input.extend_from_slice(&timestamp.to_be_bytes());
    input.extend_from_slice(&price.to_be_bytes());
    stream.write_all(&input)?;
    Ok(())
}
