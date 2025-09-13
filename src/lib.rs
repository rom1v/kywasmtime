mod interval;
mod js;
mod sleep;
mod time;
mod timeout;

/// ```
/// use std::time::Duration;
/// use kywasmtime::*;
///
/// async fn demo_sleep() {
///     let duration = Duration::from_millis(500);
///     sleep(duration).await;
/// }
///
/// async fn demo_sleep_until() {
///     let deadline = Instant::now() + Duration::from_millis(200);
///     sleep_until(deadline).await;
/// }
///
/// async fn demo_interval() {
///     let mut period = Duration::from_secs(1);
///     let mut interval = interval(period);
///     interval.tick().await;
///     println!("tick 1");
///     interval.tick().await;
///     println!("tick 2");
///     interval.tick().await;
///     println!("tick 3");
/// }
///
/// async fn demo_interval_at() {
///     let mut start = Instant::now() + Duration::from_millis(200);
///     let mut period = Duration::from_secs(1);
///     let mut interval = interval_at(start, period);
///     interval.tick().await;
///     println!("tick 1");
///     interval.tick().await;
///     println!("tick 2");
///     interval.tick().await;
///     println!("tick 3");
/// }
///
/// async fn demo_timeout() {
///     let duration = Duration::from_millis(500);
///     match timeout(duration, async move {
///         // simulate long action
///         sleep(Duration::from_millis(500));
///         42
///     }).await {
///         Ok(result) => println!("{result}"),
///         Err(_) => println!("timeout"),
///     }
/// }
///
/// async fn demo_timeout_at() {
///     let deadline = Instant::now() + Duration::from_millis(200);
///     match timeout_at(deadline, async move {
///         // simulate long action
///         sleep(Duration::from_millis(500));
///         42
///     }).await {
///         Ok(result) => println!("{result}"),
///         Err(_) => println!("timeout"),
///     }
/// }
/// ```
pub use interval::{interval, interval_at, Interval};
pub use sleep::{sleep, sleep_until, Sleep};
pub use time::{Instant, SystemTime, SystemTimeError, UNIX_EPOCH};
pub use timeout::{timeout, timeout_at, Timeout};

pub mod error {
    // Match the path of tokio::time::error::Elapsed for convenience
    pub use crate::timeout::Elapsed;
}
