# 02. Lifetimes and Reference Scopes in Rust

## Overview
Every reference in Rust has a **lifetime**, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred (Lifetime Elision), but explicit annotations (`'a`) are required when relationship scopes are ambiguous.

---

## The Borrow Checker & Dangling References
The Rust borrow checker prevents **dangling references**—accessing memory that has already been deallocated.

```rust
// Lifetime Annotations in Function Signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest string: {}", result);
    }
}
```

---

## Structs with Lifetime Annotations
If a struct holds references instead of owned values, it must declare lifetime parameters:

```rust
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = Excerpt { part: first_sentence };
    println!("Excerpt: {}", i.part);
}
```

---

## Static Lifetime (`'static`)
The `'static` lifetime duration spans the entire execution of the program (e.g. hardcoded string literals stored in binary read-only memory):

```rust
let s: &'static str = "I live forever in binary memory.";
```
