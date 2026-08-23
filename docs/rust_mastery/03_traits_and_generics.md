# 03. Traits, Polymorphism, and Generics in Rust

## Overview
Traits define shared behavior in Rust, similar to interfaces in other languages. Generics allow writing reusable code for multiple concrete types without runtime overhead.

---

## Defining and Implementing Traits

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

---

## Trait Bounds & Generics
Use trait bounds to constrain generic type parameters:

```rust
// Trait bound syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Where clause syntax for complex bounds
pub fn process_data<T, U>(t: &T, u: &U) -> String
where
    T: Summary + Clone,
    U: std::fmt::Display,
{
    format!("Item: {}, Value: {}", t.summarize(), u)
}
```

---

## Static Monomorphization vs Trait Objects (`dyn Trait`)
- **Static Dispatch (Monomorphization)**: Compiler generates specialized code for each concrete type ($0\text{ ns}$ runtime overhead).
- **Dynamic Dispatch (`Box<dyn Summary>`)**: Uses vtables for runtime polymorphism when heterogeneous collections are needed.
