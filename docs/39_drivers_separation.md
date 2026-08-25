# 39 - Drivers: Source/Device Separation (Unix Philosophy)

```
┌──────────────────────────────────────────────────────────────┐
│                    UNIX-STYLE SEPARATION                     │
│                                                              │
│   ┌─────────────────────────────────────────────┐           │
│   │              CORE (kernel)                   │           │
│   │  learn · define · eval · compose · persona  │           │
│   │  synonym · resonance · wave · save · load   │           │
│   │  stats · help · exit                        │           │
│   └──────────────────┬──────────────────────────┘           │
│                      │                                       │
│   ┌──────────────────┴──────────────────────────┐           │
│   │            DRIVERS (devices)                 │           │
│   │  ingest · ingest-json · ingest-wiktionary   │           │
│   │  chunk · train                               │           │
│   └──────────────────────────────────────────────┘           │
│                                                              │
│   Drivers are dispatched FIRST. If a line matches a driver,  │
│   it's handled there. Otherwise it falls through to core.    │
└──────────────────────────────────────────────────────────────┘
```

## Why separate?

In Unix, the kernel doesn't know about specific hardware. Device drivers
handle the interface between external sources and the kernel. Similarly:

- **Core** commands operate on the facet (the "kernel" data structure)
- **Drivers** interface with external data sources (files, APIs, dumps)
  and feed data into the facet

This means:
- Adding a new data source = adding a new driver, not touching core
- Core commands never depend on specific file formats
- The Command enum stays clean and focused

## How dispatch works

```
Dispatcher::dispatch(line, ctx)
    │
    ├── Driver::try_dispatch(line, ctx)  ← check drivers first
    │       ├── ingest       → Ingest::local()
    │       ├── ingest-json  → Ingest::json()
    │       ├── ingest-wiktionary → Ingest::wiktionary()
    │       ├── chunk        → Chunk::execute()
    │       └── train        → Train::execute()
    │
    └── (if no driver matched)
        └── Command::from_str(parts[0])  ← fall through to core
            ├── help, learn, define, eval, compose, persona, ...
            └── unknown → Learn::default() (treat as text to learn)
```

## Module structure

```
src/drivers/
├── mod.rs     - Driver enum, from_str(), try_dispatch()
├── ingest.rs  - Ingest: local, json, wiktionary handlers
├── chunk.rs   - Chunk: splits large JSON dictionaries
└── train.rs   - Train: parallel training from chunks
```

## File references

- `src/drivers/mod.rs` - Driver enum, dispatch logic
- `src/drivers/ingest.rs` - Ingest handlers (moved from command/)
- `src/drivers/chunk.rs` - Chunk handler (moved from command/)
- `src/drivers/train.rs` - Train handler (moved from command/)
- `src/command/mod.rs:86-90` - Driver dispatch in Dispatcher
