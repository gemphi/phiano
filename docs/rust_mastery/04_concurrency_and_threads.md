# 04. Concurrency, Threads, and Shared State in Rust

## Overview
Rust guarantees **Fearless Concurrency**-preventing data races at compile time using ownership, `Send`, and `Sync` traits.

---

## 1. Thread Spawning with Move Closures

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    // 'move' moves ownership of v into the spawned thread closure
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

---

## 2. Shared State with `Arc<Mutex<T>>`
To share mutable data across multiple threads safely:
- `Arc<T>`: Atomic Reference Counted pointer for multi-threaded ownership.
- `Mutex<T>`: Mutual exclusion lock protecting data access.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // Prints 10
}
```

---

## 3. Message Passing with Channels (`mpsc`)

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi from thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```
