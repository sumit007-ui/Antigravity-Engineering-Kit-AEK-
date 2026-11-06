# Global Engineering Principle: Resource-Efficient Execution

## Objective
Deliver maximum performance and minimal memory footprint, optimized for target environments with as little as 8 GB RAM.

## Performance Invariants
1. **Low Idle Footprint**: Avoid persistent background daemons where on-demand execution suffices. Maintain <50 MB idle consumption.
2. **Zero Polling Loops**: Avoid busy-wait polling loops. Use asynchronous reactive event listeners and OS notification mechanisms.
3. **Bounded Concurrency**: Restrict parallel tasks to sensible boundaries (default 2 to 4 workers) to prevent thread exhaustion or system memory thrashing.
4. **Zero-Copy & Minimal Allocations**: Use borrowed slices (`&str`, `&[u8]`) where practical. Avoid unnecessary `.clone()` calls and allocations in hot paths.
