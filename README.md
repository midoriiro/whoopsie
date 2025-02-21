# Whoopsie
**Whoopsie** is a Rust crate designed to facilitate error management in distributed systems. 
By combining backoff with a circuit breaker, **Whoopsie** allows for intelligent and efficient handling of failures.

## Key Features
 - Backoff: Automatically retries an operation after a failure, wait time between attempts.
 - Circuit Breaker: Monitors failures and, after reaching a certain threshold, prevents further attempts to avoid overwhelming a distressed service or operation.
 - Automatic Reset: When a circuit breaker closes, the backoff counter is reset to zero, ensuring a fresh start for retries.

## Feature flags
 - ```async```: through tokio runtime

## Examples
### Backoff with constant time
https://github.com/midoriiro/whoopsie/blob/f3407a6a9d8f973140931c9693978dc1bd76c269/examples/backoff_with_constant_time.rs#L1-L40
### Backoff with exponential time
https://github.com/midoriiro/whoopsie/blob/f3407a6a9d8f973140931c9693978dc1bd76c269/examples/backoff_with_exponential_time.rs#L1-L36
### Circuit breaker
https://github.com/midoriiro/whoopsie/blob/f3407a6a9d8f973140931c9693978dc1bd76c269/examples/random_http_status.rs#L7-L44
### Asynchronous
https://github.com/midoriiro/whoopsie/blob/f3407a6a9d8f973140931c9693978dc1bd76c269/examples/asynchronous.rs#L11-L50
