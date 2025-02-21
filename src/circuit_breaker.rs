use crate::backoff::{Backoff, WaitStrategy};
use crate::error::Error;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum State {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Debug, Clone)]
pub struct CircuitBreakerBuilder {
    attempts: Option<usize>,
    failure_threshold: Option<usize>,
    reset_timeout: Option<Duration>,
    backoff: Option<Backoff>,
}

impl CircuitBreakerBuilder {
    pub fn new() -> Self {
        Self {
            attempts: None,
            failure_threshold: None,
            reset_timeout: None,
            backoff: None,
        }
    }

    pub fn with_attempts(&mut self, attempts: usize) -> &mut Self {
        assert!(attempts > 0);
        self.attempts = Some(attempts);
        self
    }

    pub fn with_failure_threshold(&mut self, threshold: usize) -> &mut Self {
        assert!(threshold > 1);
        self.failure_threshold = Some(threshold);
        self
    }

    pub fn with_reset_timeout(&mut self, duration: Duration) -> &mut Self {
        self.reset_timeout = Some(duration);
        self
    }

    pub fn with_backoff(&mut self, backoff: Backoff) -> &mut Self {
        self.backoff = Some(backoff);
        self
    }

    pub fn build(&self) -> Result<CircuitBreaker, Error> {
        if self.reset_timeout.is_none() {
            return Err(Error {
                description: "Reset timeout is required".to_string(),
            })
        }
        if self.backoff.is_none() {
            return Err(Error {
                description: "Backoff is required".to_string(),
            })
        }
        Ok(CircuitBreaker {
            failed_attempts: 0,
            attempts: self.attempts.unwrap_or(1),
            failure_count: 0,
            failure_threshold: self.failure_threshold.unwrap_or(2),
            reset_timeout: self.reset_timeout.unwrap().clone(),
            wait_strategy: WaitStrategy::Synchronous,
            backoff: self.backoff.clone().unwrap(),
            state: State::Closed,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    pub(crate) failed_attempts: usize,
    attempts: usize,
    pub(crate) failure_count: usize,
    failure_threshold: usize,
    reset_timeout: Duration,
    wait_strategy: WaitStrategy,
    backoff: Backoff,
    pub(crate) state: State,
}

impl CircuitBreaker {
    pub fn reset(&mut self) {
        self.state = State::Closed;
        self.failure_count = 0;
        self.backoff.reset();
    }

    pub fn retry<F, O, E>(&mut self, operation: &mut F) -> Result<O, E>
    where
        F: FnMut() -> Result<O, E>,
        E: std::error::Error + From<Error>,
    {
        if self.state == State::Open {
            return Err(Error {
                description: "Circuit breaker is open".to_string(),
            }.into());
        }
        self.reset();
        self.state = State::Closed;
        let mut last_error = None;
        while self.failed_attempts < self.attempts {
            if self.state == State::HalfOpen {
                match operation() {
                    Ok(value) => {
                        self.state = State::Closed;
                        return Ok(value);
                    }
                    Err(error) => {
                        self.state = State::Closed;
                        self.failed_attempts += 1;
                        self.wait_strategy.synchronous_wait(&self.reset_timeout);
                        last_error = Some(error);
                    }
                }
            }
            while self.failure_count < self.failure_threshold {
                let error = match self.backoff.retry(operation) {
                    Ok(value) => {
                        self.state = State::Closed;
                        return Ok(value)
                    },
                    Err(value) => value,
                };
                last_error = Some(error);
                self.failure_count += 1;
            }
            self.state = State::HalfOpen;
            self.wait_strategy.synchronous_wait(&self.reset_timeout);
        }
        Err(last_error.unwrap())
    }
}