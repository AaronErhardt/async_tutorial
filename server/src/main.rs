use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener, runtime::Runtime, sync::broadcast
};

/// Initialize the runtime and start the main async task.
fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async_main());
}

/// Run the broadcast server
async fn async_main() {
    // ? Create TCP listener 

    // ? Wait for new connection
        // ? Wait for incoming messages
        // ? Broadcast the message
}
