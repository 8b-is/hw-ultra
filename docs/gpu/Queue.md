# GPU Queue

The queue stores up to 63 commands in 64 slots. It does not access hardware or
own the memory addressed by a command's raw data pointer.

## One owner

Use a mutable `Queue` with `enqueue` and `dequeue`, or a mutable `GpuScheduler`
with `submit` and `poll`. `Command::submit` also takes a mutable scheduler.
Enqueue returns false when full; dequeue returns None when empty.

## Two threads

Call `Queue::split` or `GpuScheduler::split` with an exclusive mutable borrow.
Move the resulting producer and consumer to separate scoped threads. Each
endpoint's operations require a mutable reference, and endpoints cannot be
cloned. The borrow prevents a second split until both endpoints are dropped.
The producer's `enqueue` returns `Err(command)` when full so the caller can retry.

Slots use individual UnsafeCells. Release/acquire cursor publication orders
slot writes and reads; the producer waits for the consumer to release a slot
before reusing it. No spin lock, allocation, or hardware call occurs in a queue
operation. Waiting or retrying is the caller's choice.

## Migration

The previous shared-reference enqueue/submit and dequeue/poll methods permitted
safe callers to race on the same slot. Change single-owner bindings to mutable
bindings. Replace shared queues with one uniquely owned endpoint per direction.
Multiple producers or consumers need external ownership coordination; do not
create duplicate endpoints or bypass the borrow checker.

This is a source-breaking API repair. Release/version coordination is pending.
LPU scheduler/pipeline ownership is a separate outstanding repair; this GPU
change does not certify those APIs or physical GPU behavior.
