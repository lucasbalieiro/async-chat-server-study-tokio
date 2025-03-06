use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("Server started at localhost:8080");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            println!("Connection received from {:?}", addr);
            let mut reader = BufReader::new(reader);

            loop {
                let mut line = String::new();

                let bytes_read = reader.read_line(&mut line).await.unwrap();

                if bytes_read == 0 {
                    break;
                }

                println!("GOT={}", line);

                writer.write_all(&line.as_bytes()).await.unwrap();

                line.clear();
            }
        });
    }
}
