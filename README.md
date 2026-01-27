Implementation of the [`tokio::time` API][tokio-time], but for WASM.

[tokio-time]: https://docs.rs/tokio/latest/tokio/time/index.html

## Usage

This lib replicates the tokio API, so users only have to change their `use`
declarations:

```rust
#[cfg(not(target_family="wasm"))]
use tokio::time::*;
#[cfg(target_family="wasm")]
use kywasmtime::*;
```


## Examples

(copied from [`src/lib.rs`](src/lib.rs))

```rust
use std::time::Duration;
use kywasmtime::*;

async fn demo_sleep() {
    let duration = Duration::from_millis(500);
    sleep(duration).await;
}

async fn demo_sleep_until() {
    let deadline = Instant::now() + Duration::from_millis(200);
    sleep_until(deadline).await;
}

async fn demo_interval() {
    let mut period = Duration::from_secs(1);
    let mut interval = interval(period);
    interval.tick().await;
    println!("tick 1");
    interval.tick().await;
    println!("tick 2");
    interval.tick().await;
    println!("tick 3");
}

async fn demo_interval_at() {
    let mut start = Instant::now() + Duration::from_millis(200);
    let mut period = Duration::from_secs(1);
    let mut interval = interval_at(start, period);
    interval.tick().await;
    println!("tick 1");
    interval.tick().await;
    println!("tick 2");
    interval.tick().await;
    println!("tick 3");
}

async fn demo_timeout() {
    let duration = Duration::from_millis(500);
    match timeout(duration, async move {
        // simulate long action
        sleep(Duration::from_millis(500));
        42
    }).await {
        Ok(result) => println!("{result}"),
        Err(_) => println!("timeout"),
    }
}

async fn demo_timeout_at() {
    let deadline = Instant::now() + Duration::from_millis(200);
    match timeout_at(deadline, async move {
        // simulate long action
        sleep(Duration::from_millis(500));
        42
    }).await {
        Ok(result) => println!("{result}"),
        Err(_) => println!("timeout"),
    }
}
```

## License

MIT License
