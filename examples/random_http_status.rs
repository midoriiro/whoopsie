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