use crate::circuit_breaker::{CircuitBreaker, State};
use crate::error::Error;
use crate::fixtures::{circuit_breaker, failed_operation, failed_operation_then_recovered_at_first_attempt, failed_operation_then_recovered_at_second_attempt, success_operation};
use rstest::rstest;

#[rstest]
fn with_success_operation(
    mut circuit_breaker: CircuitBreaker,
    mut success_operation: impl FnMut() -> Result<(), Error>
) {
    let result = circuit_breaker.retry(&mut success_operation);
    assert_eq!(true, result.is_ok());
    assert_eq!(State::Closed, circuit_breaker.state);
    assert_eq!(0, circuit_breaker.failed_attempts);
    assert_eq!(0, circuit_breaker.failure_count);
}

#[rstest]
fn with_failed_operation(
    mut circuit_breaker: CircuitBreaker,
    mut failed_operation: impl FnMut() -> Result<(), Error>
) {
    let result = circuit_breaker.retry(&mut failed_operation);
    assert_eq!(true, result.is_err());
    assert_eq!(State::HalfOpen, circuit_breaker.state);
    assert_eq!(2, circuit_breaker.failed_attempts);
    assert_eq!(2, circuit_breaker.failure_count);
}

#[rstest]
fn with_failed_operation_then_recovered_at_first_attempt(
    mut circuit_breaker: CircuitBreaker,
    mut failed_operation_then_recovered_at_first_attempt: impl FnMut() -> Result<(), Error>
) {
    let result = circuit_breaker.retry(&mut failed_operation_then_recovered_at_first_attempt);
    assert_eq!(true, result.is_ok());
    assert_eq!(State::Closed, circuit_breaker.state);
    assert_eq!(0, circuit_breaker.failed_attempts);
    assert_eq!(2, circuit_breaker.failure_count);
}

#[rstest]
fn with_failed_operation_then_recovered_at_second_attempt(
    mut circuit_breaker: CircuitBreaker,
    mut failed_operation_then_recovered_at_second_attempt: impl FnMut() -> Result<(), Error>
) {
    let result = circuit_breaker.retry(&mut failed_operation_then_recovered_at_second_attempt);
    assert_eq!(true, result.is_ok());
    assert_eq!(State::Closed, circuit_breaker.state);
    assert_eq!(1, circuit_breaker.failed_attempts);
    assert_eq!(2, circuit_breaker.failure_count);
}