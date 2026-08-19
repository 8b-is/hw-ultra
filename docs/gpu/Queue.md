# GPU Queue

## Overview

The GPU queue is a lock-free ring buffer that stores pending GPU commands before they are dispatched to hardware.

## Structure

```
Queue {
    ring: [Option<Command>; 64]   — command slots
    head: AtomicUsize              — producer index
    tail: AtomicUsize              — consumer index
}
```

## Capacity

- 64 command slots
- Maximum 63 in-flight commands (one slot reserved for full detection)

## Operations

### Enqueue (producer)

```
queue.enqueue(cmd) -> bool
```

Writes `cmd` at `ring[head % 64]`, atomically increments `head`. Returns `false` if the ring is full.

### Dequeue (consumer)

```
queue.dequeue() -> Option<Command>
```

Reads from `ring[tail % 64]`, atomically increments `tail`. Returns `None` if empty.

## Thread safety

Atomic head/tail indices allow concurrent enqueue and dequeue from different threads without locks. However, multiple producers or multiple consumers require external synchronization.
