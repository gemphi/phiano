# 05. Error Handling and Monads in Rust

## Overview
Rust does not have exceptions. Instead, it groups errors into two categories:
1. **Unrecoverable Errors (`panic!`)**: Fatal bugs where execution halts immediately.
2. **Recoverable Errors (`Result<T, E>`)**: Expected failures handled explicitly with pattern matching or the `?` operator.

---

## 1. The `Result<T, E>` Enum

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Example of handling file errors:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

---

## 2. The Question Mark (`?`) Operator for Error Propagation

The `?` operator unwraps `Ok(T)` or returns `Err(E)` immediately from the enclosing function:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

---

## 3. The `Option<T>` Monad for Null Safety

Rust has no `null`. Missing values are represented via `Option<T>`:

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}
```
