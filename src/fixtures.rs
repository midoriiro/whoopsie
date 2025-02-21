use crate::backoff::{AsynchronousExecutor, BackoffBuilder};
use crate::circuit_breaker::{CircuitBreaker, CircuitBreakerBuilder};
use crate::error::Error;
use rstest::fixture;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;

pub const TIME: Duration = Duration::from_millis(100);

#[fixture]
pub fn async_runtime() -> Arc<AsynchronousExecutor> {
    Arc::new(AsynchronousExecutor::Runtime(Runtime::new().unwrap()))
}

#[fixture]
pub fn success_operation() -> impl FnMut() -> Result<(), Error> {
    || {
        Ok(())
    }
}

#[fixture]
pub fn failed_operation() -> impl FnMut() -> Result<(), Error> {
    || {
        Err(Error {
            description: "Something went wrong.".to_string(),
        })
    }
}

#[fixture]
pub fn failed_operation_then_recovered_at_first_attempt() -> impl FnMut() -> Result<(), Error> {
    let mut attempts = 0;
    move || {
        if attempts == 2 {
            return Ok(());
        }
        attempts += 1;
        Err(Error {
            description: "Something went wrong.".to_string(),
        })
    }
}

#[fixture]
pub fn failed_operation_then_recovered_at_second_attempt() -> impl FnMut() -> Result<(), Error> {
    let mut attempts = 0;
    move || {
        if attempts == 3 {
            return Ok(());
        }
        attempts += 1;
        Err(Error {
            description: "Something went wrong.".to_string(),
        })
    }
}

#[fixture]
pub fn circuit_breaker() -> CircuitBreaker {
    CircuitBreakerBuilder::new()
        .with_attempts(2)
        .with_failure_threshold(2)
        .with_reset_timeout(TIME * 2)
        .with_backoff(BackoffBuilder::new()
            .with_constant_time(TIME)
            .as_synchronous()
            .build()
            .unwrap()
        )
        .build()
        .unwrap()
}