# 10 — Tokenizer Pipeline

```
  Raw Input: "The Cat's SAT on the mat!"

  Step 1: Lowercase
    "the cat's sat on the mat!"

  Step 2: Split on whitespace
    ["the", "cat's", "sat", "on", "the", "mat!"]

  Step 3: Strip non-alphanumeric per token
    ["the", "cats", "sat", "on", "the", "mat"]

  Step 4: Filter empty tokens
    ["the", "cats", "sat", "on", "the", "mat"]

  Result: Vec<String> of clean tokens
```

## Normalize vs Tokenize

```
  normalize():  lowercase + replace non-alnum with space + collapse
    "Hello, World!" → "hello world"

  tokenize():   lowercase + split + strip + filter
    "Hello, World!" → ["hello", "world"]

  split_sentences():  split on . ! ?
    "Hello. World! How?" → ["Hello", "World", "How"]
```

## Where Each Is Used

```
  Tokenizer::tokenize()     ← Trainer, Evaluator, Envision, Wave
  Tokenizer::normalize()    ← (available for preprocessing)
  Tokenizer::split_sentences() ← (available for batch processing)
```
