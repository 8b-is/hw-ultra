# TPU Compiler

## Overview

The TPU compiler transforms a computation `Graph` into a `CompiledGraph` — an optimized execution plan that can be run on the TPU hardware.

## CompiledGraph

```
CompiledGraph {
    sizes: [usize; 16]       — memory allocation per operation
    exec_order: [u8; 16]     — topological execution order
    count: usize              — number of operations
    total_memory: usize       — total TPU memory required
}
```

## API

| Function | Description |
|----------|-------------|
| `compile(graph)` | Compiles a graph into an execution plan |
| `compile_graph(graph)` | Alias for `compile()` |
| `CompiledGraph::new()` | Creates empty compiled graph |

## Compilation steps

1. **Topological sort** — determines execution order respecting data dependencies
2. **Memory estimation** — calculates memory needed for each operation's output
3. **Total memory** — sums all operation memory requirements

## Limits

- Maximum 16 operations per graph
- Each operation gets an entry in `sizes[]` and `exec_order[]`
