use anyhow::Result;
use tokio::{
    fs::File,
    io::{BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    println!("Args: {:?}", args);
    match args[0].as_str() {
        "send" => send(&args).await,
        "recv" => recv(&args).await,
        _ => panic!("Unsupported Command"),
    }
}

async fn send(args: &[String]) -> Result<()> {
    let path = std::path::Path::new(&args[1]);
    if !path.is_file() {
        panic!("Failed to find file: {:?}", path);
    }

    let mut file = BufReader::new(File::open(&args[1]).await?);
    let (socket, _addr) = TcpListener::bind("0.0.0.0:8080").await?.accept().await?;
    let mut socket = BufWriter::new(socket);

    tokio::io::copy_buf(&mut file, &mut socket).await?;
    Ok(())
}

async fn recv(args: &[String]) -> Result<()> {
    let mut file = BufWriter::new(File::create(&args[2]).await?);
    let mut socket = BufReader::new(TcpStream::connect(&args[1]).await?);

    tokio::io::copy_buf(&mut socket, &mut file).await?;
    Ok(())
}
