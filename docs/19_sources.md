# 19 — Dictionary Sources

## Source Hierarchy

```
  ┌─────────────────────────────────────────────────────┐
  │  DictionarySource (trait)                           │
  │                                                     │
  │  fn fetch_all() → Vec<(String, String)>            │
  │  fn fetch_definitions(word) → Vec<String>          │
  └──────────────────┬──────────────────────────────────┘
                     │
     ┌───────────────┼───────────────┬──────────────┐
     │               │               │              │
     ▼               ▼               ▼              ▼
  ┌──────┐    ┌──────────┐   ┌───────┐    ┌───────────┐
  │ API  │    │   JSON   │   │ Local │    │ Wiktionary│
  │Source│    │  Source  │   │Source │    │  Source   │
  └──────┘    └──────────┘   └───────┘    └───────────┘
     │               │               │              │
     ▼               ▼               ▼              ▼
  api.           websters_       definitions   kaikkidump
  dictionary     dictionary      .txt           .jsonl
  api.dev        .json           (word: def)    (JSONL)
```

## When Each Source Is Used

```
  ┌────────────┬─────────────────────────┬──────────────────────┐
  │ Source     │ When Used               │ Data Format           │
  ├────────────┼─────────────────────────┼──────────────────────┤
  │ ApiSource  │ define <word>           │ HTTP JSON → cache     │
  │            │ (on-demand lookup)      │ to api_cache.txt      │
  ├────────────┼─────────────────────────┼──────────────────────┤
  │ JsonSource │ ingest-json <file>      │ {"word":"def",...}    │
  │            │ (bulk ingestion)        │ Webster's format      │
  ├────────────┼─────────────────────────┼──────────────────────┤
  │ LocalSource│ ingest <file.txt>       │ word: definition      │
  │            │ define <word> (fallback)│ one per line          │
  ├────────────┼─────────────────────────┼──────────────────────┤
  │ Wiktionary │ ingest-wiktionary <f>   │ JSON or JSONL         │
  │            │ (bulk ingestion)        │ Kaikki.org format     │
  └────────────┴─────────────────────────┴──────────────────────┘
```

## Define Command Source Chain

```
  define "cat"
       │
       ▼
  ┌──────────────┐
  │ ApiSource     │──► HTTP GET api.dictionaryapi.dev
  │ (cache first) │──► check api_cache.txt
  └──────┬───────┘
         │
    found? ├─ YES → train on definitions
         │ │
         │ NO
         ▼
  ┌──────────────┐
  │ LocalSource   │──► read definitions.txt
  │ (fallback)    │──► search for "cat:"
  └──────┬───────┘
         │
    found? ├─ YES → train on definitions
         │ │
         │ NO
         ▼
  ┌──────────────┐
  │ "No defs for  │
  │  'cat'. Try:  │
  │  learn \"cat   │
  │  <definition>\"│
  └──────────────┘
```
