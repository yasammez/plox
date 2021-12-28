#![warn(rust_2018_idioms)]


use bytes::BytesMut;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8081".to_string());
    let server_addr = env::args().nth(2).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    println!("Listening on: {}", listen_addr);
    println!("Proxying to: {}", server_addr);

    let listener = TcpListener::bind(listen_addr).await?;
    while let Ok((inbound, _)) = listener.accept().await {
        let transfer = transfer(inbound, server_addr.clone()).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);
    }

    Ok(())
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let mut inbytes = BytesMut::with_capacity(8192);
    let mut outbytes = BytesMut::with_capacity(8192);

    let client_to_server = async {
        loop {
            let count = ri.read_buf(&mut inbytes).await?;
            if count == 0 { break; }
            wo.write_all(&inbytes[..count]).await?;
            stdout.write_all("\x1B[31m".as_bytes()).await?;
            stdout.write_all(&inbytes[..count]).await?;
            inbytes.clear();
        }
        wo.shutdown().await
    };

    let server_to_client = async {
        loop {
            let count = ro.read_buf(&mut outbytes).await?;
            if count == 0 { break; }
            wi.write_all(&outbytes[..count]).await?;
            stderr.write_all("\x1B[34m".as_bytes()).await?;
            stderr.write_all(&outbytes[..count]).await?;
            outbytes.clear();
        }
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}
