use std::{future::Future, panic::catch_unwind, thread};
use std::time::{Duration, Instant};
use std::pin::Pin;
use std::task::{Context, Poll};
use async_task::{Runnable, Task};
use once_cell::sync::Lazy;


static QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
    let (tx, rx) = flume::unbounded::<Runnable>();
    thread::spawn(move || {
        while let Ok(runnable) = rx.recv() {
            let _ = catch_unwind(|| runnable.run());
        }
    });
    tx
});

pub fn spawn_task<F, T>(future: F) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    // Define how the task is scheduled: send its runnable into our queue
    let schedule = |runnable: Runnable| {
        QUEUE.send(runnable).unwrap();
    };

    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    task
}


struct AsyncSleep {
    start_time: Instant,
    duration: Duration,
}

impl AsyncSleep {
    fn new(duration: Duration) -> Self {
        Self {
            start_time: Instant::now(),
            duration
        }
    }
}

impl Future for AsyncSleep {
    // Type the future will return when done (in this case nothing)
    type Output = ();

    // How the future checks if it's ready to complete or if it needs more time
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let elapsed_time = self.start_time.elapsed();
        if elapsed_time >= self.duration {    
            Poll::Ready(())
        } else {    
            cx.waker().wake_by_ref();    
            Poll::Pending
        }
    }
}

pub async fn sleeping(label: u8) {
    println!("sleeping {}", label);
    AsyncSleep::new(Duration::from_secs(3)).await;
    println!("progressing sleep {}", label);
    AsyncSleep::new(Duration::from_secs(2)).await;
    println!("done sleeping {}", label);
}


