use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use tokio::join;

struct SleepFuture {
    end_time: Instant,
}

impl SleepFuture {
    fn new(duration: Duration) -> Self {
        let end_time = Instant::now() + duration;
        Self {
            end_time,
        }
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polled");

        // 1. Version
        // -> Return ready
        todo!()

        // 2. Version
        // -> Return ready when time passed
        
        // 3. Version
        // -> Wake the runtime
    }
}

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    rt.block_on(async {
        let duration = Duration::from_secs(2);

        println!("Wait...");
        SleepFuture::new(duration).await;
        println!("Wait...");
        SleepFuture::new(duration).await;

        println!("Wait twice...");
        let delay1 = SleepFuture::new(duration);
        let delay2 = SleepFuture::new(duration);
        join!(delay1, delay2)
    });
}
