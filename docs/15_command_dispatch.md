# 15 - Command Dispatch

## Command Enum

```
  ┌─────────────────────────────────────────────────┐
  │  Command enum (17 variants)                     │
  │                                                 │
  │  Help, Learn, Define, Eval, Synonym,           │
  │  Resonance, Wave, Ingest, IngestJson,           │
  │  IngestWiktionary, Chunk, Train, Save,          │
  │  Load, Stats, Exit, Unknown                     │
  └─────────────────────────────────────────────────┘
```

## Dispatch Flow

```
  User input: "synonym cat 5"
         │
         ▼
  ┌─────────────────────┐
  │  Split on whitespace │
  │  parts = ["synonym", │
  │           "cat 5"]   │
  └──────────┬──────────┘
             │
             ▼
  ┌─────────────────────┐
  │  Command::from_str   │
  │  ("synonym")         │
  │  → Command::Synonym  │
  └──────────┬──────────┘
             │
             ▼
  ┌─────────────────────┐
  │  ctx.arg = "cat 5"   │
  └──────────┬──────────┘
             │
             ▼
  ┌─────────────────────┐
  │  match Command {    │
  │    Synonym =>        │
  │      synonym::Synonym│
  │        .execute(ctx) │
  │  }                   │
  └─────────────────────┘
```

## Alias Support

```
  Command::from_str() accepts aliases:

  "help" | "?"        → Help
  "eval" | "judge"    → Eval
  "synonym" | "synonyms" → Synonym
  "exit" | "quit"     → Exit

  Any unrecognized string → Unknown
  Unknown → Learn::default() (treats input as text to learn)
```
