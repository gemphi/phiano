# 21 - Context & Command Handlers

## Context Struct

```
  ┌──────────────────────────────────────────────┐
  │  Context<'a>                                  │
  │                                               │
  │  ┌─────────────────┐  ┌─────────────────┐   │
  │  │ manifold: &mut  │  │ trainer: &      │   │
  │  │   Facet         │  │   Trainer       │   │
  │  │  (mutable)      │  │  (immutable)    │   │
  │  └─────────────────┘  └─────────────────┘   │
  │  ┌─────────────────┐  ┌─────────────────┐   │
  │  │ memory: &mut    │  │ arg: &str       │   │
  │  │   Memo          │  │  (command args) │   │
  │  │  (mutable)      │  │                 │   │
  │  └─────────────────┘  └─────────────────┘   │
  │  ┌─────────────────┐                        │
  │  │ line: &str      │                        │
  │  │  (full input)   │                        │
  │  └─────────────────┘                        │
  └──────────────────────────────────────────────┘

  Lifetime 'a ties all references to the Model's fields.
  Created fresh for each command dispatch.
```

## Handler Pattern

```
  Each command is a unit struct:

  pub struct Synonym;

  impl Synonym {
      pub fn execute(&self, ctx: &mut Context) -> bool {
          // 1. Parse args from ctx.arg
          // 2. Use ctx.manifold, ctx.trainer, ctx.memory
          // 3. Print results
          // 4. Return true (continue) or false (exit)
      }
  }

  No trait needed - dispatch uses match on Command enum.
  Each handler is stateless (unit struct).
```

## Command → Handler Mapping

```
  Command::Help       → help::Help.execute(ctx)
  Command::Learn      → learn::Learn.execute(ctx)
  Command::Define     → define::Define.execute(ctx)
  Command::Eval       → eval::Eval.execute(ctx)
  Command::Synonym    → synonym::Synonym.execute(ctx)
  Command::Resonance  → resonance::Resonance.execute(ctx)
  Command::Wave       → wave::WaveCmd.execute(ctx)
  Command::Ingest     → ingest::Ingest.local(ctx)
  Command::IngestJson → ingest::Ingest.json(ctx)
  Command::IngestWiki → ingest::Ingest.wiktionary(ctx)
  Command::Chunk      → chunk::Chunk.execute(ctx)
  Command::Train      → train::Train.execute(ctx)
  Command::Save       → save::Save.save(ctx)
  Command::Load       → save::Save.load(ctx)
  Command::Stats      → stats::Stats.execute(ctx)
  Command::Exit       → false (stop REPL)
  Command::Unknown    → learn::Learn.default(ctx)
```
