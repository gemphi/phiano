//! The coding tasks Phi-4 is asked (docs/117): small Rust functions in
//! families. A family is one task over several element types (`sum` over
//! `i32`, `i64`, `u32`, `f64`); the reader is tested on whole families it
//! never saw, so a type variant of a test task is never in training.

pub struct Task {
    pub family: String,
    pub name: String,
    pub signature: String,
    pub does: String,
}

const NUM: [&str; 4] = ["i32", "i64", "u32", "f64"];
const SIGNED: [&str; 3] = ["i32", "i64", "f64"];
const INT: [&str; 4] = ["i32", "i64", "u32", "u64"];

fn fam(out: &mut Vec<Task>, family: &str, types: &[&str], sig: &str, does: &str) {
    for t in types {
        out.push(Task { family: family.into(), name: format!("{family}_{t}"), signature: sig.replace("T", t), does: does.into() });
    }
}

pub fn all() -> Vec<Task> {
    let mut v = Vec::new();
    fam(&mut v, "sum", &NUM, "fn sum(v: &[T]) -> T", "returns the sum of the elements");
    fam(&mut v, "product", &NUM, "fn product(v: &[T]) -> T", "returns the product of the elements");
    fam(&mut v, "max", &NUM, "fn max(v: &[T]) -> Option<T>", "returns the largest element, or None if the slice is empty");
    fam(&mut v, "min", &NUM, "fn min(v: &[T]) -> Option<T>", "returns the smallest element, or None if the slice is empty");
    fam(&mut v, "mean", &NUM, "fn mean(v: &[T]) -> Option<f64>", "returns the arithmetic mean, or None if the slice is empty");
    fam(&mut v, "positives", &SIGNED, "fn count_positive(v: &[T]) -> usize", "counts the elements greater than zero");
    fam(&mut v, "squares", &NUM, "fn sum_of_squares(v: &[T]) -> T", "returns the sum of the squares of the elements");
    fam(&mut v, "dot", &NUM, "fn dot(a: &[T], b: &[T]) -> T", "returns the dot product of two slices of equal length");
    fam(&mut v, "cumulative", &NUM, "fn cumulative(v: &[T]) -> Vec<T>", "returns the running totals of the elements");
    fam(&mut v, "reverse", &NUM, "fn reversed(v: &[T]) -> Vec<T>", "returns a new vector with the elements in reverse order");
    fam(&mut v, "clamp", &NUM, "fn clamp_all(v: &mut [T], lo: T, hi: T)", "clamps every element into the range lo..=hi in place");
    fam(&mut v, "second", &NUM, "fn second_largest(v: &[T]) -> Option<T>", "returns the second largest element, or None if there is none");
    fam(&mut v, "spread", &NUM, "fn spread(v: &[T]) -> Option<T>", "returns the largest element minus the smallest, or None if the slice is empty");
    fam(&mut v, "above", &NUM, "fn count_above(v: &[T], x: T) -> usize", "counts the elements greater than x");
    fam(&mut v, "even", &INT, "fn count_even(v: &[T]) -> usize", "counts the even elements");
    fam(&mut v, "gcd", &INT, "fn gcd(a: T, b: T) -> T", "returns the greatest common divisor of a and b");
    fam(&mut v, "prime", &INT, "fn is_prime(n: T) -> bool", "tells whether n is a prime number");
    fam(&mut v, "factorial", &INT, "fn factorial(n: T) -> T", "returns n factorial");
    fam(&mut v, "fibonacci", &INT, "fn fibonacci(n: T) -> T", "returns the n-th Fibonacci number, with fibonacci(0) = 0");
    fam(&mut v, "digits", &INT, "fn digit_sum(n: T) -> T", "returns the sum of the decimal digits of n (ignoring a sign)");
    fam(&mut v, "flip", &INT, "fn reverse_digits(n: T) -> T", "returns n with its decimal digits reversed");
    fam(&mut v, "power", &INT, "fn power(base: T, exp: u32) -> T", "returns base raised to exp by repeated squaring");
    more(&mut v);
    let s = |v: &mut Vec<Task>, f: &str, sig: &str, does: &str| fam(v, f, &["str"], sig, does);
    s(&mut v, "vowels", "fn count_vowels(s: &str) -> usize", "counts the vowels a, e, i, o, u in s, in either case");
    s(&mut v, "words", "fn reverse_words(s: &str) -> String", "returns the words of s in reverse order, separated by single spaces");
    s(&mut v, "palindrome", "fn is_palindrome(s: &str) -> bool", "tells whether s reads the same backwards, ignoring case and non-letters");
    s(&mut v, "chars", "fn char_frequency(s: &str) -> std::collections::HashMap<char, usize>", "counts how often each character occurs in s");
    s(&mut v, "capital", "fn capitalize_words(s: &str) -> String", "upper-cases the first letter of every word in s");
    s(&mut v, "longest", "fn longest_word(s: &str) -> Option<&str>", "returns the longest word in s, or None if there is none");
    s(&mut v, "wordcount", "fn word_count(s: &str) -> std::collections::HashMap<String, usize>", "counts how often each word occurs in s, in lower case");
    s(&mut v, "caesar", "fn caesar(s: &str, shift: u8) -> String", "shifts every ASCII letter of s by shift places in the alphabet, wrapping around");
    s(&mut v, "stack", "struct Stack<T> with new, push, pop, peek, len and is_empty", "is a last-in first-out stack backed by a Vec");
    s(&mut v, "queue", "struct Queue<T> with new, push, pop, len and is_empty", "is a first-in first-out queue backed by a VecDeque");
    s(&mut v, "counter", "struct Counter with new, add(&str), count(&str) -> usize and most_common() -> Option<(String, usize)>", "counts words");
    s(&mut v, "point", "struct Point { x: f64, y: f64 } with new, distance(&self, other: &Point) -> f64 and midpoint", "is a point in the plane");
    s(&mut v, "matrix", "struct Matrix2 { a: [[f64; 2]; 2] } with identity, mul(&self, other: &Matrix2) -> Matrix2 and det", "is a 2 by 2 matrix");
    v
}

/// The second round (docs/118): families that resemble the first ones in
/// their names and shapes (`sum_even` beside `sum` and `count_even`), so a
/// student who learns by family resemblance has neighbours to learn from.
fn more(v: &mut Vec<Task>) {
    let rows: [(&str, &[&str], &str, &str); 37] = [
        ("sum_even", &INT, "fn sum_even(v: &[T]) -> T", "returns the sum of the even elements"),
        ("sum_odd", &INT, "fn sum_odd(v: &[T]) -> T", "returns the sum of the odd elements"),
        ("count_odd", &INT, "fn count_odd(v: &[T]) -> usize", "counts the odd elements"),
        ("count_negative", &SIGNED, "fn count_negative(v: &[T]) -> usize", "counts the elements less than zero"),
        ("count_zero", &NUM, "fn count_zero(v: &[T]) -> usize", "counts the elements equal to zero"),
        ("count_below", &NUM, "fn count_below(v: &[T], x: T) -> usize", "counts the elements less than x"),
        ("count_equal", &NUM, "fn count_equal(v: &[T], x: T) -> usize", "counts the elements equal to x"),
        ("sum_above", &NUM, "fn sum_above(v: &[T], x: T) -> T", "returns the sum of the elements greater than x"),
        ("contains", &NUM, "fn contains(v: &[T], x: T) -> bool", "tells whether x is one of the elements"),
        ("index_of", &NUM, "fn index_of(v: &[T], x: T) -> Option<usize>", "returns the index of the first element equal to x"),
        ("all_positive", &SIGNED, "fn all_positive(v: &[T]) -> bool", "tells whether every element is greater than zero"),
        ("any_negative", &SIGNED, "fn any_negative(v: &[T]) -> bool", "tells whether some element is less than zero"),
        ("all_even", &INT, "fn all_even(v: &[T]) -> bool", "tells whether every element is even"),
        ("any_even", &INT, "fn any_even(v: &[T]) -> bool", "tells whether some element is even"),
        ("first_even", &INT, "fn first_even(v: &[T]) -> Option<T>", "returns the first even element"),
        ("max_index", &NUM, "fn max_index(v: &[T]) -> Option<usize>", "returns the index of the largest element"),
        ("min_index", &NUM, "fn min_index(v: &[T]) -> Option<usize>", "returns the index of the smallest element"),
        ("doubled", &NUM, "fn doubled(v: &[T]) -> Vec<T>", "returns every element times two"),
        ("squared", &NUM, "fn squared(v: &[T]) -> Vec<T>", "returns the square of every element"),
        ("keep_even", &INT, "fn evens(v: &[T]) -> Vec<T>", "returns the even elements in order"),
        ("keep_positive", &SIGNED, "fn positives(v: &[T]) -> Vec<T>", "returns the elements greater than zero in order"),
        ("keep_above", &NUM, "fn above(v: &[T], x: T) -> Vec<T>", "returns the elements greater than x in order"),
        ("sorted", &NUM, "fn is_sorted(v: &[T]) -> bool", "tells whether the elements are in non-decreasing order"),
        ("max_abs", &SIGNED, "fn max_abs(v: &[T]) -> Option<T>", "returns the largest absolute value"),
        ("sum_abs", &SIGNED, "fn sum_abs(v: &[T]) -> T", "returns the sum of the absolute values"),
        ("is_even", &INT, "fn is_even(n: T) -> bool", "tells whether n is even"),
        ("is_odd", &INT, "fn is_odd(n: T) -> bool", "tells whether n is odd"),
        ("abs_diff", &INT, "fn abs_diff(a: T, b: T) -> T", "returns the absolute difference of a and b"),
        ("lcm", &INT, "fn lcm(a: T, b: T) -> T", "returns the least common multiple of a and b"),
        ("count_digits", &INT, "fn count_digits(n: T) -> u32", "counts the decimal digits of n (ignoring a sign)"),
        ("sum_to", &INT, "fn sum_to(n: T) -> T", "returns 1 + 2 + ... + n"),
        ("product_to", &INT, "fn product_to(n: T) -> T", "returns 1 * 2 * ... * n"),
        ("square", &NUM, "fn square(n: T) -> T", "returns n times n"),
        ("cube", &NUM, "fn cube(n: T) -> T", "returns n times n times n"),
        ("max_of", &NUM, "fn max_of(a: T, b: T) -> T", "returns the larger of a and b"),
        ("min_of", &NUM, "fn min_of(a: T, b: T) -> T", "returns the smaller of a and b"),
        ("triangle", &INT, "fn triangle(n: T) -> T", "returns the n-th triangular number"),
    ];
    for (f, ty, sig, does) in rows {
        fam(v, f, ty, sig, does);
    }
    let words: [(&str, &str, &str); 11] = [
        ("consonants", "fn count_consonants(s: &str) -> usize", "counts the ASCII letters of s that are not vowels"),
        ("spaces", "fn count_spaces(s: &str) -> usize", "counts the space characters in s"),
        ("digits_in", "fn count_digits_in(s: &str) -> usize", "counts the decimal digit characters in s"),
        ("uppercase", "fn count_uppercase(s: &str) -> usize", "counts the upper-case letters in s"),
        ("reverse_string", "fn reverse_string(s: &str) -> String", "returns s with its characters in reverse order"),
        ("first_word", "fn first_word(s: &str) -> Option<&str>", "returns the first word of s"),
        ("last_word", "fn last_word(s: &str) -> Option<&str>", "returns the last word of s"),
        ("word_lengths", "fn word_lengths(s: &str) -> Vec<usize>", "returns the length of every word of s, in order"),
        ("remove_spaces", "fn remove_spaces(s: &str) -> String", "returns s without its space characters"),
        ("remove_vowels", "fn remove_vowels(s: &str) -> String", "returns s without the vowels a, e, i, o, u in either case"),
        ("anagram", "fn is_anagram(a: &str, b: &str) -> bool", "tells whether a and b have the same letters the same number of times"),
    ];
    for (f, sig, does) in words {
        fam(v, f, &["str"], sig, does);
    }
}

/// The words of the task names, each with a name it occurs in (docs/120).
pub fn words() -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for t in all() {
        let name = t.signature.split([' ', '(', '<']).nth(1).unwrap_or("").to_string();
        for w in name.split('_').filter(|w| w.len() > 1) {
            if !out.iter().any(|(x, _)| x == w) {
                out.push((w.to_string(), name.clone()));
            }
        }
    }
    out
}

/// The tokens a word may be glossed by.
pub const GLOSSES: [&str; 24] = ["+", "-", "*", "/", "%", "<", ">", "<=", ">=", "==", "!=", "&&", "||", "!", "abs", "max", "min", "sum", "product", "count", "filter", "rev", "len", "none"];

/// The question for one word: which code token says it.
pub fn gloss_prompt(word: &str, example: &str) -> String {
    format!(
        "In Rust code, which single token best expresses the word \"{word}\" as it is used in a function name like `{example}`? Choose one of: {}. Reply with the token only.",
        GLOSSES.join(" ")
    )
}

/// The question for one word's expression (docs/121): what the word says
/// about a value `x` (and a bound `y` when it needs one), as Rust.
pub fn expr_prompt(word: &str, example: &str) -> String {
    format!(
        "In Rust, the word \"{word}\" in a function name like `{example}` tests or computes something about a number x (and a second number y, if the word needs one). \
Write that as one Rust expression over x (and y). For example, for \"nonzero\" write `x != 0`; for \"larger\" write `x > y`; for \"halved\" write `x / 2`. Reply with the expression only, no words."
    )
}

/// The question for a word the student does not know (docs/124): the word
/// shown in use, as the builders' game shows a colour by pointing.
pub fn uses_prompt(word: &str, example: &str) -> String {
    format!(
        "Write three different short Rust functions over numbers whose names contain the word \"{word}\", as in `{example}`, each doing what its name says. Reply with only the Rust code in one ```rust block: no explanation, no tests, no main function."
    )
}

/// The question for one task: the signature and what it does, nothing else.
pub fn prompt(t: &Task) -> String {
    format!(
        "Write Rust code for this: `{}`. It {}. Reply with only the Rust code in one ```rust block: no explanation, no tests, no main function.",
        t.signature, t.does
    )
}
