use tokio;
use tokio::net::TcpListener;
use tokio::io::{    AsyncWriteExt, BufReader, AsyncBufReadExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    loop {

        // Readline just appends what it reads, does not overwrite.
        let bytes_read = reader.read_line(&mut line).await.unwrap();

        if bytes_read == 0 {
            break;
        }

        writer.write_all(line.as_bytes()).await.unwrap();
        line.clear();
    }


}
