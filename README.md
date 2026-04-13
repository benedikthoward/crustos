# cRusTOS

A configurable, type-safe RTOS written in Rust with FreeRTOS-extracted port assembly and declarative TOML configuration.

## What is cRusTOS?

cRusTOS is a real-time operating system that combines battle-tested FreeRTOS port assembly with a pure Rust kernel. Instead of the 65+ `#define` macros that make FreeRTOS configuration unreadable, cRusTOS uses a single `crustos.toml` file. Instead of `#ifdef` spaghetti, cRusTOS uses Rust's trait system to make every major policy decision — scheduling algorithm, memory allocation strategy, tick source, tracing backend — swappable at compile time with zero runtime cost.

The key insight: FreeRTOS port assembly (PendSV handlers, SVC handlers, context switch code) is only ~50 lines per architecture. The other ~700 lines in each port file are C code that can be replaced with shared, readable Rust. cRusTOS keeps the proven assembly and rewrites everything else.

## Architecture

### Repo structure

```
crustos/
├── crustos/              # The RTOS library
│   ├── src/
│   │   ├── lib.rs
│   │   ├── config.rs
│   │   ├── task.rs
│   │   ├── scheduler.rs
│   │   ├── sync/         # Mutexes, semaphores, queues, channels, etc.
│   │   ├── tracing.rs
│   │   └── port/
│   │       ├── mod.rs    # Port trait definition
│   │       ├── arm/
│   │       │   ├── v7m/  # ARMv7-M (Cortex-M3, M4F, M7)
│   │       │   └── v8m/  # ARMv8-M (Cortex-M23, M33, M55)
│   │       ├── riscv/
│   │       └── xtensa/
├── crustos-macros/       # Proc macros (init!, #[task])
├── crustos-extract/      # CLI tool to auto-extract FreeRTOS port asm
└── examples/
    └── stm32f103-blinky/
```

### Configuration

Users place a `crustos.toml` in their project root. The `init!` macro reads it at compile time and resolves all type aliases and feature flags:

```toml
[port]
arch = "cortex-m3"

[scheduler]
algorithm = "fixed-priority"
max_priorities = 8

[memory]
allocator = "free-list"
heap_size = 4096

[tick]
source = "systick"
rate_hz = 1000

[tracing]
preset = "context-switches"
backend = "rtt"
```


### Swappable traits

Every major policy decision is a trait resolved at compile time from the TOML config. No runtime dispatch, no generics infecting the kernel — just concrete type aliases.

**Port** — hardware architecture. Four core asm blocks per target (context switch handler, restore first task, start first task, request context switch) plus interrupt masking, stack init, and tick timer setup.

**Scheduler** — task selection policy. Owns the ready-state data structures internally. The kernel tells it about state changes (task ready, task blocked, task yielded) and asks it to pick the next task.

**Allocator** — memory strategy. Fixed allocation, pool, free-list, or multi-region.

**TickSource** — time base. SysTick, dedicated hardware timer, or custom.

**TraceBackend** — event interface. The kernel emits trace events at hook points; the user wires transport via their HAL.

### Kernel design

The kernel manages task lifecycle (delayed, suspended, terminated lists) while the scheduler manages ready-state ordering. This separation means the kernel doesn't bake in assumptions about scheduling policy.

The kernel uses intrusive doubly-linked lists. Each TCB embeds two `ListNode`s — one for its position in a kernel lifecycle list (delayed, suspended, etc.) and one for its position in a sync primitive's wait list. A task can be in both simultaneously, enabling blocking-with-timeout on any sync primitive.

Lists support four insertion modes: `insert_head` (O(1), for priority inheritance boost), `insert_end` (O(1), normal ready insertion), `insert_sorted` (O(n), by value for kernel delayed list), and `insert_sorted_by_sched_ctx` (O(n), by scheduler-defined ordering).

### Scheduling algorithms

All compile-time selected via TOML. The `Scheduler` trait means anyone can implement a custom algorithm by defining a struct and a trait impl.

**Fixed-priority preemptive** — higher priority always runs. Array of ready lists indexed by priority. v0.1 target.

**Round-robin** — fixed-priority between levels, time-sliced within a level.

**Earliest Deadline First (EDF)** — single sorted ready list by absolute deadline. Periodic task support with automatic job release.

**Constant Bandwidth Server (CBS)** — EDF with budget tracking. When a server exhausts its budget, its deadline is pushed forward rather than the task being blocked. Enables soft real-time aperiodic tasks alongside hard real-time periodic tasks.

**Earliest Eligible Virtual Deadline First (EEVDF)** — fair scheduling with virtual time tracking. Eligible tasks sorted by virtual deadline.

Priority inheritance is handled through the scheduler trait. The kernel tells the scheduler "this task is blocking that task" via `task_blocks_on`, and the scheduler decides what "boost" means for its algorithm — raising priority for fixed-priority, adopting an earlier deadline for EDF, etc.

### IPC and synchronization

All sync primitives are standalone kernel objects. Tasks block on them via the existing `event_list_node` mechanism.

**Ownership-based (priority inheritance):** mutexes (with compile-time priority ceiling via proc macros), RwLocks.

**Signaling:** semaphores (binary + counting), event groups (bitmask AND/OR), condition variables, task notifications (direct-to-task, lightweight).

**Message passing:** queues (fixed-size FIFO), mailboxes (single-item overwrite), typed channels (Sender\<T\>/Receiver\<T\> with ownership transfer), stream buffers (byte stream), message buffers (variable-length framed messages).

**Lock-free:** ring buffers (single-producer single-consumer, ISR-safe).

**Compile-time only:** resource guards (priority ceiling enforcement via the type system, zero runtime cost).

### Async support (v0.3)

Async/await with a preemptive scheduler underneath — the "Embassy but with preemption" story.

Each async task runs an executor as a normal preemptible cRusTOS task. Between tasks, scheduling is preemptive with hard real-time guarantees. Within a task, sequencing is cooperative async/await.

Wakers are built on top of the signaling infrastructure (task notifications and event groups). When a hardware interrupt fires, it wakes the corresponding future and marks the owning task as ready. If that task is higher priority than what's currently running, preemption is immediate.

### Extractor tool

`crustos-extract` is a CLI tool that parses FreeRTOS port directories, rewrites symbols to cRusTOS equivalents, and emits Rust files with `global_asm!()` blocks. This gives cRusTOS automatic support for every architecture FreeRTOS supports (~40+) without manually porting each one.

The extractor only needs to handle ~50 lines of assembly per architecture. Everything else is shared Rust code.

## Roadmap

**v0.1** — Hand-ported Cortex-M3, basic kernel, fixed-priority preemptive scheduler, mutexes with priority inheritance and compile-time priority ceiling, tracing presets, TOML config, blinky on STM32F103.

**v0.2** — Round-robin, EDF, CBS, and EEVDF schedulers. Full sync primitive suite. Extractor tool. Additional architecture ports.

**v0.3** — Async/await layer with preemptive scheduling. Hardware-tied wakers. Async-aware sync primitives.

## License

MIT — FreeRTOS port assembly is also MIT licensed.