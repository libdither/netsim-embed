use std::process::exit;
use std::str::from_utf8;

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
// use futures::{AsyncReadExt, AsyncWriteExt};

async fn process(stream: TcpStream) -> io::Result<()> {
    println!("Accepted from: {}", stream.peer_addr()?);

	let mut lines = io::BufReader::new(stream.clone()).lines();
    let mut writer = stream;

	while let Some(line) = lines.next().await {
		let mut line = line?;
		println!("Echoing: {:?}", line);
		if line == "exit" { exit(0) }
		line += "\n";
		writer.write_all(line.as_bytes()).await?;
	}

    Ok(())
}

fn main() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("0.0.0.0:3000").await?;
        println!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                process(stream).await.unwrap();
            });
        }
        Ok(())
    })
}