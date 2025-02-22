use crate::error::Error;
use std::future::Future;
use std::time::Duration;

#[derive(Debug, Clone)]
pub(crate) enum TimeStrategy {
    Constant {
        duration: Duration,
    },
    Exponential {
        initial_duration: Duration,
        duration: Duration,
        factor: f32,
    }
}

#[derive(Debug, Clone)]
pub(crate) enum WaitStrategy {
    Synchronous,
    #[cfg(feature = "async")]
    Asynchronous,
    SpinLoop
}

impl WaitStrategy {
    pub fn synchronous_wait(&self, duration: &Duration) {
        std::thread::sleep(*duration);
    }

    #[cfg(feature = "async")]
    pub async fn asynchronous_wait(&self, duration: &Duration) {
        tokio::time::sleep(duration.clone()).await;
    }

    pub fn spin_loop_wait(&self, duration: &Duration) {
        let end_time = std::time::Instant::now() + *duration;
        while std::time::Instant::now() < end_time {
            std::hint::spin_loop();
        }
    }
}

impl TimeStrategy {
    pub fn reset(&mut self) {
        match self {
            TimeStrategy::Constant { .. } => {}
            TimeStrategy::Exponential { initial_duration, duration, ..} => {
                *duration = initial_duration.clone();
            }
        }
    }
}

pub struct BackoffBuilder {
    time_strategy: Option<TimeStrategy>,
    wait_strategy: Option<WaitStrategy>
}

impl BackoffBuilder {
    pub fn new() -> Self {
        Self {
            time_strategy: None,
            wait_strategy: None,
        }
    }

    pub fn with_constant_time(&mut self, duration: Duration) -> &mut Self {
        self.time_strategy = Some(TimeStrategy::Constant {
            duration,
        });
        self
    }

    pub fn with_exponential_time(&mut self, duration: Duration, factor: f32) -> &mut Self {
        self.time_strategy = Some(TimeStrategy::Exponential {
            initial_duration: duration.clone(),
            duration,
            factor,
        });
        self
    }

    pub fn as_synchronous(&mut self) -> &mut Self {
        self.wait_strategy = Some(WaitStrategy::Synchronous);
        self
    }

    #[cfg(feature = "async")]
    pub fn as_asynchronous(&mut self) -> &mut Self {
        self.wait_strategy = Some(WaitStrategy::Asynchronous);
        self
    }

    pub fn as_spin_loop(&mut self) -> &mut Self {
        self.wait_strategy = Some(WaitStrategy::SpinLoop);
        self
    }

    pub fn build(&self) -> Result<Backoff, Error> {
        if self.time_strategy.is_none() {
            return Err(Error {
                description: "Time strategy is required. Call one of 'with_*' method".to_string(),
            })
        }
        if self.wait_strategy.is_none() {
            return Err(Error {
                description: "Wait strategy is required. Call one of 'as_*' method".to_string(),
            })
        }
        Ok(Backoff::new(
            self.time_strategy.as_ref().unwrap().clone(),
            self.wait_strategy.as_ref().unwrap().clone()
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Backoff {
    pub(crate) time_strategy: TimeStrategy,
    pub(crate) wait_strategy: WaitStrategy,
}

impl Backoff {
    fn new(
        time_strategy: TimeStrategy,
        wait_strategy: WaitStrategy,
    ) -> Self {
        Self {
            time_strategy,
            wait_strategy,
        }
    }

    pub fn reset(&mut self) {
        self.time_strategy.reset();
    }

    pub fn retry<F, O, E>(&mut self, operation: &mut F) -> Result<O, E>
    where
        F: FnMut() -> Result<O, E>,
        E: std::error::Error,
    {
        let error = match operation() {
            Ok(value) => return Ok(value),
            Err(value) => value
        };
        let duration = Self::get_next(&self.time_strategy);
        self.synchronous_wait(duration);
        Self::compute_next(&mut self.time_strategy);
        Err(error)
    }

    #[cfg(feature = "async")]
    pub async fn retry_async<F, O, E, R>(&mut self, operation: &mut F) -> Result<O, E>
    where
        F: FnMut() -> R,
        E: std::error::Error,
        R: Future<Output = Result<O, E>>,
    {
        let error = match operation().await {
            Ok(value) => return Ok(value),
            Err(value) => value
        };
        let duration = Self::get_next(&self.time_strategy);
        self.asynchronous_wait(duration).await;
        Self::compute_next(&mut self.time_strategy);
        Err(error)
    }

    pub(crate) fn get_next(time_strategy: &TimeStrategy) -> &Duration {
        match time_strategy {
            TimeStrategy::Constant { ref duration } => {
                duration
            }
            TimeStrategy::Exponential {
                initial_duration: _initial_duration,
                ref duration,
                factor: _factor
            } => {
                duration
            }
        }
    }

    fn compute_next(time_strategy: &mut TimeStrategy) {
        match time_strategy {
            TimeStrategy::Exponential {
                initial_duration: _initial_duration,
                ref mut duration,
                factor
            } => {
                *duration = duration.mul_f32(*factor);
            }
            _ => {}
        };
    }

    fn synchronous_wait(&self, duration: &Duration) {
        match self.wait_strategy {
            WaitStrategy::Synchronous => {
                self.wait_strategy.synchronous_wait(duration);
            }
            #[cfg(feature = "async")]
            WaitStrategy::Asynchronous => {
                panic!("Asynchronous wait not supported in synchronous context");
            }
            WaitStrategy::SpinLoop => {
                self.wait_strategy.spin_loop_wait(duration);
            }
        }
    }

    #[cfg(feature = "async")]
    async fn asynchronous_wait(&self, duration: &Duration) {
        match &self.wait_strategy {
            WaitStrategy::Asynchronous => {
                let duration = duration.clone();
                let strategy = self.wait_strategy.clone();
                strategy.asynchronous_wait(&duration).await
            }
            WaitStrategy::Synchronous | WaitStrategy::SpinLoop => {
                panic!("Synchronous wait not supported in asynchronous context");
            }
        }
    }
}