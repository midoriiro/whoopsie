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