# GPU Command and Queue

## Command

```
Command {
    opcode: u32         — GPU command opcode
    data_ptr: *const u8 — pointer to command data
    data_len: usize     — data length in bytes
}
```

| Method | Description |
|--------|-------------|
| `new(op, data, len)` | Creates a command |
| `nop()` | Creates a no-operation command |
| `is_nop()` | Returns true if this is a NOP |
| `submit(sched)` | Submits to the GPU scheduler |

## Queue

```
Queue {
    ring: [Option<Command>; 64]
    head: AtomicUsize
    tail: AtomicUsize
}
```

Lock-free ring buffer with 64 slots.

| Method | Description |
|--------|-------------|
| `new()` | Creates an empty queue |
| `enqueue(cmd)` | Adds command, returns false if full |
| `dequeue()` | Removes and returns next command |

### Ring operation

- `enqueue` writes at `head % 64`, increments `head`
- `dequeue` reads at `tail % 64`, increments `tail`
- Full when `(head + 1) % 64 == tail`
- Empty when `head == tail`

## Scheduler

The GPU scheduler pulls commands from the queue and submits them to the hardware command processor. Commands are executed in FIFO order.
