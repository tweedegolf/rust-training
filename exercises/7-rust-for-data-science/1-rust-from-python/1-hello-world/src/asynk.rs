use std::{
    borrow::Borrow,
    pin::Pin,
    task::{Context, Poll},
    thread::JoinHandle,
    time::{Duration, SystemTime},
};

use pyo3::prelude::*;

/// A `Future` that spawns a thread which sleeps
/// for the given duration, and resolves after the thread
/// is finished
///
/// You probably don't want to use something like this,
/// as spawning a thread is quite heavy.
/// But at least we don't have to depend on `tokio` or
/// similar to demonstrate a sleep future.
struct Sleep {
    duration: Duration,
    sleep_thread: Option<JoinHandle<()>>,
}

impl std::future::Future for Sleep {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(t) = &self.sleep_thread {
            if t.is_finished() {
                return Poll::Ready(());
            }
        } else {
            let waker = cx.waker().clone();
            let duration = std::mem::take(&mut self.duration);
            let st = std::thread::spawn(move || {
                std::thread::sleep(duration);
                waker.wake();
            });
            self.sleep_thread.replace(st);
        }

        Poll::Pending
    }
}

fn sleep(duration: Duration) -> Sleep {
    Sleep {
        duration,
        sleep_thread: None,
    }
}

#[pyfunction]
pub async fn print_sleep(duration: Duration) {
    let start = SystemTime::now();
    println!("🌙 Night night! Sleeping for {:?}", duration);
    sleep(duration).await;
    println!(
        "🌞 I'm awake after {:?}",
        SystemTime::now().duration_since(start).unwrap()
    );
}

// #[pyfunction]
// async fn does_not_compile<'py>(arg: Bound<'py, PyAny>) -> Bound<'py, PyAny> {
//     todo!()
// }

#[pyfunction]
async fn does_compile(arg: Py<PyAny>) -> Py<PyAny> {
    Python::with_gil(|py| { // (almost) no-op
        let r: &Bound<'_, PyAny> = arg.bind(py);

        todo!()
    })
}
