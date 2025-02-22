# Whoopsie
**Whoopsie** is a Rust crate designed to facilitate error management in distributed systems. 
By combining backoff with a circuit breaker, **Whoopsie** allows for intelligent and efficient handling of failures.

## Key Features
 - Backoff: Automatically retries an operation after a failure, wait time between ATTEMPTS.
 - Circuit Breaker: Monitors failures and, after reaching a certain threshold, prevents further ATTEMPTS to avoid overwhelming a distressed service or operation.
 - Automatic Reset: When a circuit breaker closes, the backoff counter is reset to zero, ensuring a fresh start for retries.

## Feature flags
 - ```async```: through tokio runtime

## Examples
### Backoff with constant time
<!-- MARKDOWN-AUTO-DOCS:START (CODE:src=./examples/backoff_with_constant_time.rs) -->
<!-- MARKDOWN-AUTO-DOCS:END -->
### Backoff with exponential time
<!-- MARKDOWN-AUTO-DOCS:START (CODE:src=./examples/backoff_with_exponential_time.rs) -->
<!-- MARKDOWN-AUTO-DOCS:END -->
### Circuit breaker
<!-- MARKDOWN-AUTO-DOCS:START (CODE:src=./examples/random_http_status.rs) -->
<!-- MARKDOWN-AUTO-DOCS:END -->
### Asynchronous
<!-- MARKDOWN-AUTO-DOCS:START (CODE:src=./examples/asynchronous.rs) -->
<!-- MARKDOWN-AUTO-DOCS:END -->
