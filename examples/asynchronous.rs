use std::sync::{Arc, LazyLock, Mutex};
use std::time::Duration;
use whoopsie::backoff::BackoffBuilder;
use whoopsie::circuit_breaker::CircuitBreakerBuilder;
use whoopsie::error::Error;

static ATTEMPTS: LazyLock<Arc<Mutex<usize>>> = LazyLock::new(|| Arc::new(Mutex::new(0)));

async fn brewing_coffee() -> Result<&'static str, Error>{
    let mut attempts = ATTEMPTS.lock().unwrap();
    *attempts += 1;
    println!("☕ Attempt #{}: Brewing coffee...", attempts);

    let failure_scenarios = [
        None,
        Some("🚰 No water!"),
        Some("⚡ Power outage!"),
        Some("🛠️ Coffee grinder jammed!"),
    ];

    match failure_scenarios.get(*attempts % failure_scenarios.len()) {
        Some(Some(error_message)) => Err(Error {
            description: error_message.to_string(),
        }),
        _ => {
            // Need time to pour coffee into coffee cup.
            tokio::time::sleep(Duration::from_millis(100)).await;
            // Yeah, it's a large coffee cup, need more time.
            tokio::time::sleep(Duration::from_millis(100)).await;
            // I know, this is absurd.
            tokio::time::sleep(Duration::from_millis(100)).await;
            // And to be honest I prefer russian earl grey.
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok("✅ Fresh coffee is ready! Enjoy! ☕")
        },
    }
}

#[cfg(feature = "async")]
#[tokio::main]
async fn main() {
    let backoff = BackoffBuilder::new()
        .with_constant_time(Duration::from_millis(100))
        .as_asynchronous()
        .build()
        .unwrap();

    let mut circuit_breaker = CircuitBreakerBuilder::new()
        .with_attempts(2)
        .with_failure_threshold(2)
        .with_reset_timeout(Duration::from_millis(100))
        .with_backoff(backoff)
        .build()
        .unwrap();
    
    let mut coffee_machine = || brewing_coffee();

    let result = match circuit_breaker.retry_async(&mut coffee_machine).await {
        Ok(message) => format!("{}", message),
        Err(error) => format!("❌ Coffee machine gave up: {}", error),
    };

    println!("{}", result);
}