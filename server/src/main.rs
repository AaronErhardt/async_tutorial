use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    runtime::Runtime,
    sync::broadcast,
};

/// Initialize the runtime and start the main async task.
fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async_main());
}

/// Run the broadcast server
async fn async_main() {
    let listener = TcpListener::bind("127.0.0.1:7777").await.unwrap();
    let (sender, _receiver) = broadcast::channel(1);

    while let Ok((stream, _addr)) = listener.accept().await {
        let (mut read_handle, mut write_handle) = stream.into_split();

        let sender = sender.clone();
        let mut receiver = sender.subscribe();
        tokio::spawn(async move {
            loop {
                let length = read_handle.read_u64().await.unwrap();
                let mut buf = vec![0; length as usize];
                read_handle.read_exact(&mut buf).await.unwrap();
                sender.send(buf).unwrap();
            }
        });

        tokio::spawn(async move {
            while let Ok(buf) = receiver.recv().await {
                write_handle.write_u64(buf.len() as u64).await.unwrap();
                write_handle.write_all(&buf).await.unwrap();
            }
        });
    }
}

#[tokio::test]
async fn test() {}
