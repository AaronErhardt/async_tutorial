use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use tokio::join;

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

        // let now = Instant::now();
        // let duration = self.end_time - now;
        // std::thread::sleep(duration);
        // return Poll::Ready(());

        if Instant::now() > self.end_time {
            Poll::Ready(())
        } else {
            if !self.thread_spawned {
                self.thread_spawned = true;
                let end_time = self.end_time;
                let waker = cx.waker().clone();

                std::thread::spawn(move || {
                    let now = Instant::now();
                    let duration = end_time - now;
                    std::thread::sleep(duration);
                    waker.wake()
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
        join!(delay1, delay2)
    });
}
