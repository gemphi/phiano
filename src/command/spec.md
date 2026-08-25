# Command Module Specification

## Overview

The command module handles REPL command parsing and dispatch. Each command
is implemented as a struct with an `execute` method that receives a
mutable `Context`.

## Key Types

### `Context<'a>`
Shared context passed to every command handler:
- `manifold: &'a mut Facet` - mutable access to the lexicon
- `trainer: &'a Trainer` - shared trainer for learning
- `memory: &'a mut Memo` - mutable memory log
- `arg: &'a str` - command arguments (after the command name)
- `line: &'a str` - full raw input line

### `Command` (enum)
All recognized commands, parsed via `Command::from_str()`:
- `Help`, `Learn`, `Define`, `Eval`, `Synonym`, `Resonance`, `Wave`
- `Ingest`, `IngestJson`, `IngestWiktionary`, `Chunk`, `Train`
- `Save`, `Load`, `Stats`, `Exit`, `Unknown`

### `Dispatcher`
Routes input lines to command handlers via pattern matching on `Command`.

### `Parser`
Utility for stripping quotes from command arguments.

## Command Handlers

| Command     | Struct    | Method     | Description |
|-------------|-----------|------------|-------------|
| help        | Help      | execute    | Print available commands |
| learn       | Learn     | execute    | Train on a sentence |
| (default)   | Learn     | default    | Train + eval + envision |
| define      | Define    | execute    | Fetch and learn a word's definition |
| eval        | Eval      | execute    | Score text quality |
| synonym     | Synonym   | execute    | Find nearest resonant words |
| resonance   | Resonance | execute    | Find words resonating with text |
| wave        | WaveCmd   | execute    | Display sentence wave |
| ingest      | Ingest    | local      | Ingest local text file |
| ingest-json | Ingest    | json       | Ingest JSON dictionary |
| ingest-wik  | Ingest    | wiktionary | Ingest Wiktionary dump |
| chunk       | Chunk     | execute    | Split dictionary into chunks |
| train       | Train     | execute    | Train from chunks in parallel |
| save        | Save      | save       | Persist facet to disk |
| load        | Save      | load       | Load facet from disk |
| stats       | Stats     | execute    | Show facet + memory statistics |

## Design Rules

1. Each command is a unit struct (no state)
2. `execute` returns `bool` - `true` to continue REPL, `false` to stop
3. Unrecognized input falls through to `Learn::default()` (treats input as text to learn)
4. Commands use `Parser::strip_quotes` for quoted arguments
5. All commands print their own output with `println!`
