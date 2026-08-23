# 26 — REPL Interface

## rustyline Integration

```
  ┌──────────────────────────────────────────────────┐
  │  phiano> _                                       │
  │                                                  │
  │  Features:                                       │
  │  ├── Line editing (arrow keys, backspace)        │
  │  ├── History (up/down arrows)                    │
  │  ├── Ctrl-C → interrupt                          │
  │  ├── Ctrl-D → EOF → exit                         │
  │  └── Tab → (no completion, just inserts tab)     │
  └──────────────────────────────────────────────────┘
```

## Main Loop

```
  fn run(&mut self) {
      let mut rl = Editor::new();

      loop {
          match rl.readline("phiano> ") {
              Ok(raw) => {
                  let line = raw.trim().to_string();
                  if line.is_empty() { continue; }

                  rl.add_history_entry(&line);
                  self.iterate(&line);

                  if line is "exit" or "quit" {
                      break;
                  }
              }
              Err(_) => break,  // EOF or error
          }
      }
  }
```

## Startup Banner

```
  ╔══════════════════════════════════════════════════╗
  ║  PHIANO — Chromatic Resonance Agent              ║
  ║  Recursive learning: envision → apply → eval     ║
  ║                   → iterate → scale              ║
  ╚══════════════════════════════════════════════════╝

  [loaded] 30214 words
  Vocabulary: 30214 words

  Commands:
    learn "text"               — Train on a sentence
    define <word>              — Fetch & learn a definition
    ...
    exit                       — Save and quit

  phiano> _
```

## Exit Flow

```
  User types "exit"
       │
       ▼
  Model::iterate()
       │
       ├─► cmd == "exit" → self.scale()
       │                    │
       │                    ├─► Storage::save() → manifold.chroma
       │                    ├─► Memo::save_to_file() → memory.chroma
       │                    └─► print "[saved]"
       │
       └─► return (breaks loop)
              │
              ▼
  println!("  Goodbye.")
```
