use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener, runtime::Runtime, sync::broadcast
};

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async_main());
}

async fn async_main() {
    let listener = TcpListener::bind("127.0.0.1:7777").await.unwrap();
    let (sender, _receiver) = broadcast::channel(1);

    while let Ok((stream, _)) = listener.accept().await {
        let (mut read_handle, mut write_handle) = stream.into_split();

        let sender = sender.clone();
        let mut receiver = sender.subscribe();

        tokio::spawn(async move {
            let length = read_handle.read_u64().await.unwrap();
            let mut message = vec![0; length as usize];
            read_handle.read_exact(&mut message).await.unwrap();

            sender.send(message).unwrap();
        });

        tokio::spawn(async move {
            while let Ok(message) = receiver.recv().await {
                let length = message.len() as u64;
                write_handle.write_u64(length).await.unwrap();
                write_handle.write_all(&message).await.unwrap();
            }
        });

    }
}
