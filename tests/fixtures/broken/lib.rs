// A deliberate syntax error so `cargo test --no-run` fails to build. The
// spawn test reads the first `error:` line and asserts build-failed is
// distinct from a test failure (R1.9).
pub fn oops() -> i32 {
    let x = ;
    x
}
