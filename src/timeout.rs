use crate::sleep::{sleep, sleep_until, Sleep};
use crate::time::Instant;
use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Timeout<T: Future> {
    sleep: Sleep,
    future: Pin<Box<T>>,
}

impl<T: Future> Timeout<T> {
    fn new(sleep: Sleep, future: T) -> Self {
        Self {
            sleep,
            future: Box::pin(future),
        }
    }
}

impl<T: Future> Future for Timeout<T> {
    type Output = Result<T::Output, Elapsed>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut();
        let fut = this.future.as_mut();
        if let Poll::Ready(value) = fut.poll(cx) {
            return Poll::Ready(Ok(value));
        }

        let fut = &mut self.get_mut().sleep;
        let fut = Pin::new(fut);
        match fut.poll(cx) {
            Poll::Ready(_) => Poll::Ready(Err(Elapsed)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Elapsed; // Error returned by Timeout

impl std::fmt::Display for Elapsed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "deadline has elapsed")
    }
}

pub fn timeout_at<F>(deadline: Instant, future: F) -> Timeout<F::IntoFuture>
where
    F: IntoFuture,
{
    let future = future.into_future();
    let sleep = sleep_until(deadline);
    Timeout::new(sleep, future)
}

pub fn timeout<F>(duration: Duration, future: F) -> Timeout<F::IntoFuture>
where
    F: IntoFuture,
{
    let future = future.into_future();
    let sleep = sleep(duration);
    Timeout::new(sleep, future)
}
