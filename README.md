# Whoopsie
**Whoopsie** is a Rust crate designed to facilitate error management in distributed systems. 
By combining backoff with a circuit breaker, **Whoopsie** allows for intelligent and efficient handling of failures.

## Key Features
 - Backoff: Automatically retries an operation after a failure, wait time between ATTEMPTS.
 - Circuit Breaker: Monitors failures and, after reaching a certain threshold, prevents further ATTEMPTS to avoid overwhelming a distressed service or operation.
 - Automatic Reset: When a circuit breaker closes, the backoff counter is reset to zero, ensuring a fresh start for retries.

## Install

Add the following to your Cargo.toml file

```
[dependencies]
whoopsie = "0.1"
```

## Feature flags
 - ```async```: through tokio runtime

## Examples
### Backoff with constant time
<!-- MARKDOWN-AUTO-DOCS:START (CODE:src=./examples/backoff_with_constant_time.rs) -->
<!-- The below code snippet is automatically added from ./examples/backoff_with_constant_time.rs -->
```rs
use rand::Rng;
use std::time::Duration;
use whoopsie::backoff::BackoffBuilder;
use whoopsie::error::Error;

fn main() {
    let mut backoff = BackoffBuilder::new()
        .with_constant_time(Duration::from_millis(100))
        .as_synchronous()
        .build()
        .unwrap();

    let mut rng = rand::rng();
    let mut attempts = 0;

    let mut launch_rocket = || {
        attempts += 1;
        let chance: f64 = rng.random();

        println!("🚀 Launch attempt #{}", attempts);

        if chance < 0.3 {
            Ok("🌎 Liftoff successful! Next stop: Mars!")
        } else if chance < 0.6 {
            Err(Error {
                description: "🛠️ Engine failure detected!".to_string(),
            })
        } else {
            Err(Error {
                description: "🌩️ Bad weather conditions, launch aborted!".to_string(),
            })
        }
    };

    let result = match backoff.retry(&mut launch_rocket) {
        Ok(message) => format!("✅ Mission success: {}", message),
        Err(error) => format!("❌ Mission aborted after multiple failures: {}", error),
    };

    println!("{}", result);
}
```
<!-- MARKDOWN-AUTO-DOCS:END -->
### Backoff with exponential time
<!-- MARKDOWN-AUTO-DOCS:START (CODE:src=./examples/backoff_with_exponential_time.rs) -->
<!-- The below code snippet is automatically added from ./examples/backoff_with_exponential_time.rs -->
```rs
use rand::Rng;
use std::time::Duration;
use whoopsie::backoff::BackoffBuilder;
use whoopsie::error::Error;

fn main() {
    let mut backoff = BackoffBuilder::new()
        .with_exponential_time(Duration::from_millis(100), 2.0)
        .as_synchronous()
        .build()
        .unwrap();

    let mut rng = rand::rng();

    let mut operation = || {
        let number = rng.random_range(0..=4);
        match number {
            0 | 1 => Ok("Rust is the best! Memory safety and zero-cost abstractions!"),
            2 => Err(Error {
                description: "C++ enters the chat: 'Did someone say performance?'".to_string(),
            }),
            3 => Err(Error {
                description: "Python interrupts: 'But readability matters!'".to_string(),
            }),
            _ => Err(Error {
                description: "C# says: 'Hey, have you tried .NET? It's pretty cool too!'".to_string(),
            }),
        }
    };

    let result = match backoff.retry(&mut operation) {
        Ok(value) => format!("✅ Success: {}", value),
        Err(error) => format!("❌ Failure: {}", error),
    };

    println!("{}", result);
}
```
<!-- MARKDOWN-AUTO-DOCS:END -->
### Circuit breaker
<!-- MARKDOWN-AUTO-DOCS:START (CODE:src=./examples/random_http_status.rs) -->
<!-- The below code snippet is automatically added from ./examples/random_http_status.rs -->
```rs
use std::time::Duration;
use whoopsie::backoff::BackoffBuilder;
use whoopsie::circuit_breaker::CircuitBreakerBuilder;
use whoopsie::error::Error;

fn main() {
    let backoff = BackoffBuilder::new()
        .with_exponential_time(Duration::from_millis(10), 2.0)
        .as_synchronous()
        .build()
        .unwrap();
    let mut circuit_breaker = CircuitBreakerBuilder::new()
        .with_attempts(2)
        .with_failure_threshold(10)
        .with_reset_timeout(Duration::from_millis(100))
        .with_backoff(backoff)
        .build()
        .unwrap();
    let mut operation = || {
        let response = reqwest::blocking::get("https://httpstat.us/random/200,400-410,500-510")
            .unwrap();
        if response.status().is_success() {
            Ok(response.text().unwrap())
        }
        else {
            Err(response.error_for_status()
                .map_err(|error| {
                    Error {
                        description: error.to_string(),
                    }
                })
                .unwrap_err()
            )
        }
    };
    let result = circuit_breaker.retry(&mut operation);
    if result.is_err() {
        println!("Error: {:?}", result.unwrap_err());
    }
    else {
        let response = result.unwrap();
        println!("Success: {}", response);   
    }
    println!("Circuit breaker status: {}", circuit_breaker);
}
```
<!-- MARKDOWN-AUTO-DOCS:END -->
### Asynchronous
<!-- MARKDOWN-AUTO-DOCS:START (CODE:src=./examples/asynchronous.rs) -->
<!-- The below code snippet is automatically added from ./examples/asynchronous.rs -->
```rs
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
```
<!-- MARKDOWN-AUTO-DOCS:END -->
