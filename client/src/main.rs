use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    join,
    net::TcpStream,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Create stream and split into read and write handles
    let stream = TcpStream::connect("127.0.0.1:7777").await.unwrap();
    let (mut read_handle, mut write_handle) = stream.into_split();

    // Wait for incoming messages
    let read_task = tokio::spawn(async move {
        loop {
            let length = read_handle.read_u64().await.unwrap();
            let mut message = vec![0; length as usize];
            read_handle.read_exact(&mut message).await.unwrap();

            let decoded_message = String::from_utf8(message).unwrap();
            println!("Received message: {decoded_message}");
        }
    });

    // Read stdio and send messages to the server
    let write_task = tokio::spawn(async move {
        let stdin = stdin();
        let mut stdin = BufReader::new(stdin);

        loop {
            println!("Write a message:");
            let mut message = String::new();
            stdin.read_line(&mut message).await.unwrap();

            let message = message.into_bytes();
            let length = message.len() as u64;
            write_handle.write_u64(length).await.unwrap();
            write_handle.write_all(&message).await.unwrap();
        }
    });

    // Make sure the runtime isn't dropped to keep the tasks alive
    let _ = join!(read_task, write_task);
}
