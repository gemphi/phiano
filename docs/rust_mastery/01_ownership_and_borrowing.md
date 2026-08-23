# 01. Ownership and Borrowing in Rust

## Overview
Ownership is Rust's most fundamental feature. It enables memory safety and memory management at compile time without requiring a garbage collector.

---

## The Three Rules of Ownership
1. **Each value in Rust has an owner** (a variable).
2. **There can only be one owner at a time**.
3. **When the owner goes out of scope, the value is dropped** (freed).

```rust
fn main() {
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1; // Ownership MOVED to s2. s1 is no longer valid!
    
    // println!("{}", s1); // Compile Error: borrow of moved value: `s1`
    println!("{}", s2); // Works! s2 owns the String memory.
}
```

---

## References and Borrowing
Instead of transferring ownership, Rust allows **borrowing** via references:

- **Immutable Borrow (`&T`)**: Multiple immutable references can exist simultaneously.
- **Mutable Borrow (`&mut T`)**: Only **ONE** active mutable reference is allowed at a time in a scope.

```rust
fn calculate_length(s: &String) -> usize { // Borrowed reference
    s.len()
} // s goes out of scope, but since it's a reference, the underlying data is NOT dropped.

fn modify_string(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut greeting = String::from("Hello");
    modify_string(&mut greeting);
    let len = calculate_length(&greeting);
    println!("{} (len: {})", greeting, len);
}
```

---

## Phiano Phase Manifold Mapping
In Phiano's phase space, ownership and borrowing concepts align in **Sector 16 (Emerald/Green)**:

$$\theta_{\text{ownership}} \approx 1.9959\text{ rad}, \quad R_{\text{coherence}} = 1.0000$$

Words like `ownership`, `borrow`, `move`, `scope`, and `drop` lock together under Kuramoto phase attraction.
