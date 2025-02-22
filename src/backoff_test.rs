use crate::backoff::{Backoff, BackoffBuilder};
use crate::error::Error;
use crate::fixtures::{failed_operation, failed_operation_async, success_operation, success_operation_async, TIME};
use rstest::rstest;
use std::future::Future;
use std::pin::Pin;

#[rstest]
fn missing_with_time_method_in_builder() {
    let backoff = BackoffBuilder::new()
        .as_synchronous()
        .build();
    assert_eq!("Time strategy is required. Call one of 'with_*' method", backoff.unwrap_err().description);
}

#[rstest]
fn missing_with_as_method_in_builder() {
    let backoff = BackoffBuilder::new()
        .with_constant_time(TIME)
        .build();
    assert_eq!("Wait strategy is required. Call one of 'as_*' method", backoff.unwrap_err().description);
}

#[rstest]
fn with_constant_time(
    mut failed_operation: impl FnMut() -> Result<(), Error>
) {
    let mut backoff = BackoffBuilder::new()
        .with_constant_time(TIME)
        .as_synchronous()
        .build()
        .unwrap();
    let _ = backoff.retry(&mut failed_operation);
    assert_eq!(TIME, *Backoff::get_next(&backoff.time_strategy));
    let _ = backoff.retry(&mut failed_operation);
    assert_eq!(TIME, *Backoff::get_next(&backoff.time_strategy));
}

#[rstest]
fn with_constant_time_then_reset(
    mut failed_operation: impl FnMut() -> Result<(), Error>
) {
    let mut backoff = BackoffBuilder::new()
        .with_constant_time(TIME)
        .as_synchronous()
        .build()
        .unwrap();
    let _ = backoff.retry(&mut failed_operation);
    assert_eq!(TIME, *Backoff::get_next(&backoff.time_strategy));
    backoff.reset();
    let _ = backoff.retry(&mut failed_operation);
    assert_eq!(TIME, *Backoff::get_next(&backoff.time_strategy));
}

#[rstest]
fn with_exponential_time(
    mut failed_operation: impl FnMut() -> Result<(), Error>
) {
    let mut backoff = BackoffBuilder::new()
        .with_exponential_time(TIME, 2.0)
        .as_synchronous()
        .build()
        .unwrap();
    let _ = backoff.retry(&mut failed_operation);
    assert!(((TIME * 2).as_millis() - (Backoff::get_next(&backoff.time_strategy).as_millis())) < 1);
    let _ = backoff.retry(&mut failed_operation);
    assert!(((TIME * 4).as_millis() - (Backoff::get_next(&backoff.time_strategy).as_millis())) < 1);
}

#[rstest]
fn with_exponential_time_then_reset(
    mut failed_operation: impl FnMut() -> Result<(), Error>
) {
    let mut backoff = BackoffBuilder::new()
        .with_exponential_time(TIME, 2.0)
        .as_synchronous()
        .build()
        .unwrap();
    let _ = backoff.retry(&mut failed_operation);
    assert!(((TIME * 2).as_millis() - (Backoff::get_next(&backoff.time_strategy).as_millis())) < 1);
    backoff.reset();
    let _ = backoff.retry(&mut failed_operation);
    assert!(((TIME * 2).as_millis() - (Backoff::get_next(&backoff.time_strategy).as_millis())) < 1);
}

#[rstest]
fn success_with_constant_time_and_as_sync(mut success_operation: impl FnMut() -> Result<(), Error>) {
    let mut backoff = BackoffBuilder::new()
        .with_constant_time(TIME)
        .as_synchronous()
        .build()
        .unwrap();
    let result = backoff.retry(&mut success_operation);
    assert_eq!(true, result.is_ok());
}

#[rstest]
fn failed_with_constant_time_and_as_sync(mut failed_operation: impl FnMut() -> Result<(), Error>) {
    let mut backoff = BackoffBuilder::new()
        .with_constant_time(TIME)
        .as_synchronous()
        .build()
        .unwrap();
    let result = backoff.retry(&mut failed_operation);
    assert_eq!(true, result.is_err());
}

#[rstest]
#[tokio::test]
async fn success_with_constant_time_and_as_async(
    mut success_operation_async: impl FnMut() -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'static>>,
) {
    let mut backoff = BackoffBuilder::new()
        .with_constant_time(TIME)
        .as_asynchronous()
        .build()
        .unwrap();
    let result = backoff.retry_async(&mut success_operation_async).await;
    assert_eq!(true, result.is_ok());
}


#[rstest]
#[tokio::test]
async fn failed_with_constant_time_and_as_async(
    mut failed_operation_async: impl FnMut() -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'static>>,
) {
    let mut backoff = BackoffBuilder::new()
        .with_constant_time(TIME)
        .as_asynchronous()
        .build()
        .unwrap();
    let result = backoff.retry_async(&mut failed_operation_async).await;
    assert_eq!(true, result.is_err());
}

#[rstest]
fn success_with_constant_time_and_as_spin_loop(mut success_operation: impl FnMut() -> Result<(), Error>) {
    let mut backoff = BackoffBuilder::new()
        .with_constant_time(TIME)
        .as_spin_loop()
        .build()
        .unwrap();
    let result = backoff.retry(&mut success_operation);
    assert_eq!(true, result.is_ok());
}

#[rstest]
fn failed_with_constant_time_and_as_spin_loop(mut failed_operation: impl FnMut() -> Result<(), Error>) {
    let mut backoff = BackoffBuilder::new()
        .with_constant_time(TIME)
        .as_spin_loop()
        .build()
        .unwrap();
    let result = backoff.retry(&mut failed_operation);
    assert_eq!(true, result.is_err());
}