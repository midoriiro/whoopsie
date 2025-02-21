**Whoopsie** is a Rust crate designed to facilitate error management in distributed systems. 
By combining exponential backoff with a circuit breaker, **Whoopsie** allows for intelligent and efficient handling of failures.

Key Features:

 - Backoff: Automatically retries an operation after a failure, wait time for a certain amount of time between attempts.
 - Circuit Breaker: Monitors failures and, after reaching a certain threshold, prevents further attempts to avoid overwhelming a distressed service.
 - Automatic Reset: When a circuit breaker closes, the backoff counter is reset to zero, ensuring a fresh start for retries.