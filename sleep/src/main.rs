use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use tokio::join;

#[must_use]
struct SleepFuture {
    end_time: Instant,
    thread_spawned: bool,
}

impl SleepFuture {
    fn new(duration: Duration) -> Self {
        let end_time = Instant::now() + duration;
        Self {
            end_time,
            thread_spawned: false,
        }
    }
}

impl Future for SleepFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polled");

        if Instant::now() > self.end_time {
            Poll::Ready(())
        } else {
            if !self.thread_spawned {
                self.thread_spawned = true;

                let waker = cx.waker().clone();
                let end_time = self.end_time;
                std::thread::spawn(move || {
                    let now = Instant::now();
                    std::thread::sleep(end_time - now);
                    waker.wake();
                });
            }
            Poll::Pending
        }
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
        join!(delay1, delay2);
    });
}
