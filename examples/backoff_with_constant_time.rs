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

    let mut rng = rand::thread_rng();
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
