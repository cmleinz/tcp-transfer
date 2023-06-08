use anyhow::Result;
use tokio::{
    fs::File,
    io::{BufReader, BufWriter},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::args().len() != 4 {
        println!("Usage:\ntcp-transfer send FILE_SOURCE LOCAL_SOCKET\ntcp-transfer recv EXTERNAL_SOCKET FILE_DESTINATION");
        return Ok(());
    }
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
    let (socket, addr) = TcpListener::bind(&args[2]).await?.accept().await?;
    println!("Accepted connection from: {addr:?}");
    let mut socket = BufWriter::new(socket);

    println!("Beginning transmit of: {}", args[1]);
    tokio::io::copy_buf(&mut file, &mut socket).await?;
    Ok(())
}

async fn recv(args: &[String]) -> Result<()> {
    let mut socket = BufReader::new(TcpStream::connect(&args[1]).await?);
    println!("Established connections to: {}", args[1]);
    let mut file = BufWriter::new(File::create(&args[2]).await?);

    println!("Beginning save of: {file:?}");
    let recv_time = std::time::Instant::now();
    tokio::io::copy_buf(&mut socket, &mut file).await?;
    if let Ok(meta) = file.into_inner().metadata().await {
        println!(
            "Successfully copied: {}\n\tRun duration: {:.2}(s)\n\tFile size: {} bytes",
            args[2],
            recv_time.elapsed().as_secs_f64(),
            meta.len()
        );
    }
    Ok(())
}
