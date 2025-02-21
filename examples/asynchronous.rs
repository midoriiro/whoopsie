use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Handle;
use whoopsie::backoff::{AsynchronousExecutor, BackoffBuilder};
use whoopsie::circuit_breaker::CircuitBreakerBuilder;
use whoopsie::error::Error;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() {
    let backoff = BackoffBuilder::new()
        .with_constant_time(Duration::from_millis(100))
        .as_asynchronous(Arc::new(AsynchronousExecutor::Handle(Handle::current())))
        .build()
        .unwrap();

    let mut circuit_breaker = CircuitBreakerBuilder::new()
        .with_attempts(2)
        .with_failure_threshold(2)
        .with_reset_timeout(Duration::from_millis(100))
        .with_backoff(backoff)
        .build()
        .unwrap();

    let mut attempts = 0;
    let mut coffee_machine = || {
        attempts += 1;
        println!("☕ Attempt #{}: Brewing coffee...", attempts);

        let failure_scenarios = [
            None,
            Some("🚰 No water!"),
            Some("⚡ Power outage!"),
            Some("🛠️ Coffee grinder jammed!"),
        ];

        match failure_scenarios.get(attempts % failure_scenarios.len()) {
            Some(Some(error_message)) => Err(Error {
                description: error_message.to_string(),
            }),
            _ => Ok("✅ Fresh coffee is ready! Enjoy! ☕"),
        }
    };

    let result = match circuit_breaker.retry_async(&mut coffee_machine).await {
        Ok(message) => format!("{}", message),
        Err(error) => format!("❌ Coffee machine gave up: {}", error),
    };

    println!("{}", result);
}