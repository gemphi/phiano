//! Does it work? One check per family of `code_quest` (docs/118): the
//! written function plus a `main` of asserts, compiled and run (2 s limit).
//! The same checks are run on Phi-4's own answers, so the student is scored
//! against the teacher on the same ground.

use super::tasks::Task;

fn asserts(t: &Task) -> Option<String> {
    let ty = t.name.rsplit('_').next().unwrap_or("i32");
    let n = |x: &str| format!("({x} as {ty})");
    // A further assert for the signed types only (negatives tell sum from sum_abs).
    let signed = |x: &str| if matches!(ty, "i32" | "i64" | "f64") { x.to_string() } else { String::new() };
    let v = |xs: &[&str]| format!("[{}]", xs.iter().map(|x| n(x)).collect::<Vec<_>>().join(", "));
    Some(match t.family.as_str() {
        "sum" => format!("assert!(sum(&{}) == {}); {}", v(&["2", "3", "4"]), n("9"), signed(&format!("assert!(sum(&{}) == {});", v(&["-5", "2"]), n("-3")))),
        "product" => format!("assert!(product(&{}) == {});", v(&["2", "3", "4"]), n("24")),
        "max" => format!("assert!(max(&{}) == Some({})); assert!(max(&[]).is_none()); {}", v(&["3", "9", "2"]), n("9"), signed(&format!("assert!(max(&{}) == Some({}));", v(&["-7", "3"]), n("3")))),
        "min" => format!("assert!(min(&{}) == Some({})); assert!(min(&[]).is_none());", v(&["3", "9", "4", "2"]), n("2")),
        "mean" => format!("assert!(mean(&{}) == Some(4.0)); assert!(mean(&[]).is_none());", v(&["1", "2", "3", "10"])),
        "positives" => format!("assert!(count_positive(&{}) == 3);", v(&["-3", "2", "4", "5"])),
        "squares" => format!("assert!(sum_of_squares(&{}) == {});", v(&["1", "2", "3"]), n("14")),
        "dot" => format!("assert!(dot(&{}, &{}) == {});", v(&["1", "2", "3"]), v(&["4", "5", "6"]), n("32")),
        "cumulative" => format!("assert!(cumulative(&{}) == vec!{});", v(&["1", "2", "3"]), v(&["1", "3", "6"])),
        "reverse" => format!("assert!(reversed(&{}) == vec!{});", v(&["1", "2", "3"]), v(&["3", "2", "1"])),
        "clamp" => format!("let mut w = {}; clamp_all(&mut w, {}, {}); assert!(w == {});", v(&["1", "5", "9"]), n("2"), n("8"), v(&["2", "5", "8"])),
        "second" => format!("assert!(second_largest(&{}) == Some({})); assert!(second_largest(&{}).is_none());", v(&["3", "9", "2"]), n("3"), v(&["1"])),
        "spread" => format!("assert!(spread(&{}) == Some({}));", v(&["3", "9", "2"]), n("7")),
        "above" => format!("assert!(count_above(&{}, {}) == 2);", v(&["1", "5", "9"]), n("4")),
        "even" => format!("assert!(count_even(&{}) == 3);", v(&["1", "2", "4", "6", "7"])),
        "gcd" => format!("assert!(gcd({}, {}) == {} && gcd({}, {}) == {});", n("12"), n("18"), n("6"), n("7"), n("21"), n("7")),
        "prime" => format!("assert!(is_prime({}) && !is_prime({}) && !is_prime({}));", n("7"), n("9"), n("1")),
        "factorial" => format!("assert!(factorial({}) == {});", n("5"), n("120")),
        "fibonacci" => format!("assert!(fibonacci({}) == {} && fibonacci({}) == {} && fibonacci({}) == {});", n("0"), n("0"), n("10"), n("55"), n("5"), n("5")),
        "digits" => format!("assert!(digit_sum({}) == {});", n("1234"), n("10")),
        "flip" => format!("assert!(reverse_digits({}) == {});", n("1234"), n("4321")),
        "power" => format!("assert!(power({}, 4) == {});", n("3"), n("81")),
        "sum_even" => format!("assert!(sum_even(&{}) == {});", v(&["1", "2", "3", "4"]), n("6")),
        "sum_odd" => format!("assert!(sum_odd(&{}) == {});", v(&["1", "2", "3", "4"]), n("4")),
        "count_odd" => format!("assert!(count_odd(&{}) == 3);", v(&["1", "2", "3", "5"])),
        "count_negative" => format!("assert!(count_negative(&{}) == 3);", v(&["-1", "-2", "-4", "3"])),
        "count_zero" => format!("assert!(count_zero(&{}) == 3);", v(&["0", "1", "0", "0", "2"])),
        "count_below" => format!("assert!(count_below(&{}, {}) == 2);", v(&["1", "5", "9"]), n("6")),
        "count_equal" => format!("assert!(count_equal(&{}, {}) == 2);", v(&["1", "5", "1"]), n("1")),
        "sum_above" => format!("assert!(sum_above(&{}, {}) == {});", v(&["1", "5", "9"]), n("4"), n("14")),
        "contains" => format!("assert!(contains(&{0}, {1}) && !contains(&{0}, {2}));", v(&["1", "5", "9"]), n("5"), n("4")),
        "index_of" => format!("assert!(index_of(&{0}, {1}) == Some(2) && index_of(&{0}, {2}).is_none());", v(&["3", "5", "9"]), n("9"), n("4")),
        "all_positive" => format!("assert!(all_positive(&{}) && all_positive(&{}) && !all_positive(&{}));", v(&["1", "2"]), v(&["3", "1"]), v(&["1", "-2"])),
        "any_negative" => format!("assert!(any_negative(&{}) && !any_negative(&{}));", v(&["1", "-2"]), v(&["1", "2"])),
        "all_even" => format!("assert!(all_even(&{}) && !all_even(&{}));", v(&["2", "4"]), v(&["2", "3"])),
        "any_even" => format!("assert!(any_even(&{}) && !any_even(&{}));", v(&["1", "4"]), v(&["1", "3"])),
        "first_even" => format!("assert!(first_even(&{}) == Some({}));", v(&["7", "4", "6", "3"]), n("4")),
        "max_index" => format!("assert!(max_index(&{}) == Some(1));", v(&["3", "9", "2"])),
        "min_index" => format!("assert!(min_index(&{}) == Some(2));", v(&["3", "9", "2"])),
        "doubled" => format!("assert!(doubled(&{}) == vec!{});", v(&["1", "3"]), v(&["2", "6"])),
        "squared" => format!("assert!(squared(&{}) == vec!{});", v(&["1", "2", "3"]), v(&["1", "4", "9"])),
        "keep_even" => format!("assert!(evens(&{}) == vec!{});", v(&["1", "2", "4", "7"]), v(&["2", "4"])),
        "keep_positive" => format!("assert!(positives(&{}) == vec!{});", v(&["-1", "2", "3"]), v(&["2", "3"])),
        "keep_above" => format!("assert!(above(&{}, {}) == vec!{});", v(&["1", "5", "9"]), n("4"), v(&["5", "9"])),
        "sorted" => format!("assert!(is_sorted(&{}) && !is_sorted(&{}) && !is_sorted(&{}));", v(&["1", "2", "2", "5"]), v(&["2", "1"]), v(&["3", "1", "2"])),
        "max_abs" => format!("assert!(max_abs(&{}) == Some({}));", v(&["-7", "3"]), n("7")),
        "sum_abs" => format!("assert!(sum_abs(&{}) == {});", v(&["-1", "2", "-4"]), n("7")),
        "is_even" => format!("assert!(is_even({}) && is_even({}) && !is_even({}));", n("4"), n("0"), n("3")),
        "is_odd" => format!("assert!(is_odd({}) && is_odd({}) && is_odd({}) && !is_odd({}));", n("3"), n("9"), n("1"), n("4")),
        "abs_diff" => format!("assert!(abs_diff({}, {}) == {} && abs_diff({}, {}) == {});", n("3"), n("8"), n("5"), n("8"), n("3"), n("5")),
        "lcm" => format!("assert!(lcm({}, {}) == {});", n("4"), n("6"), n("12")),
        "count_digits" => format!("assert!(count_digits({}) == 4);", n("1234")),
        "sum_to" => format!("assert!(sum_to({}) == {});", n("4"), n("10")),
        "product_to" => format!("assert!(product_to({}) == {});", n("4"), n("24")),
        "square" => format!("assert!(square({}) == {});", n("7"), n("49")),
        "cube" => format!("assert!(cube({}) == {});", n("3"), n("27")),
        "max_of" => format!("assert!(max_of({}, {}) == {});", n("3"), n("8"), n("8")),
        "min_of" => format!("assert!(min_of({}, {}) == {});", n("3"), n("8"), n("3")),
        "triangle" => format!("assert!(triangle({}) == {});", n("4"), n("10")),
        "consonants" => "assert!(count_consonants(\"Hello World\") == 7);".into(),
        "spaces" => "assert!(count_spaces(\"a b c\") == 2);".into(),
        "digits_in" => "assert!(count_digits_in(\"a1b22\") == 3);".into(),
        "uppercase" => "assert!(count_uppercase(\"Hello World\") == 2);".into(),
        "reverse_string" => "assert!(reverse_string(\"abc\") == \"cba\");".into(),
        "first_word" => "assert!(first_word(\"hello big world\") == Some(\"hello\"));".into(),
        "last_word" => "assert!(last_word(\"hello big go\") == Some(\"go\"));".into(),
        "word_lengths" => "assert!(word_lengths(\"a bb ccc\") == vec![1, 2, 3]);".into(),
        "remove_spaces" => "assert!(remove_spaces(\"a b c\") == \"abc\");".into(),
        "remove_vowels" => "assert!(remove_vowels(\"hello\") == \"hll\");".into(),
        "anagram" => "assert!(is_anagram(\"listen\", \"silent\") && !is_anagram(\"ab\", \"abc\"));".into(),
        "vowels" => "assert!(count_vowels(\"Hello World\") == 3);".into(),
        "words" => "assert!(reverse_words(\"ab cd\") == \"cd ab\");".into(),
        "palindrome" => "assert!(is_palindrome(\"A man, a plan, a canal: Panama\") && !is_palindrome(\"abc\"));".into(),
        "chars" => "assert!(char_frequency(\"aab\")[&'a'] == 2);".into(),
        "capital" => "assert!(capitalize_words(\"hello world\") == \"Hello World\");".into(),
        "longest" => "assert!(longest_word(\"a bbb cc\") == Some(\"bbb\"));".into(),
        "wordcount" => "assert!(word_count(\"a b a\")[\"a\"] == 2);".into(),
        "caesar" => "assert!(caesar(\"abc\", 1) == \"bcd\");".into(),
        _ => return None,
    })
}

/// `Some(true)` when the code compiles with the family's asserts and runs
/// them without a panic; `None` when the family has no check.
pub fn works(t: &Task, code: &str) -> Option<bool> {
    let a = asserts(t)?;
    let dir = std::env::temp_dir().join(format!("code_check_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let (src, exe) = (dir.join("c.rs"), dir.join("c.bin"));
    std::fs::write(&src, format!("#![allow(warnings)]\n{code}\nfn main() {{ {a} }}\n")).ok()?;
    let built = std::process::Command::new("rustc").args(["--edition", "2021", "-O", "-o"]).arg(&exe).arg(&src).stderr(std::process::Stdio::null()).status().is_ok_and(|s| s.success());
    if !built {
        return Some(false);
    }
    let mut child = std::process::Command::new(&exe).stderr(std::process::Stdio::null()).spawn().ok()?;
    let t0 = std::time::Instant::now();
    loop {
        if let Ok(Some(s)) = child.try_wait() {
            return Some(s.success());
        }
        if t0.elapsed().as_secs_f64() > 2.0 {
            let _ = child.kill();
            return Some(false);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

/// The teacher's row: Phi-4's own answers (`(family, path)`) through the same checks.
pub fn teacher(fns: &[Task], phi4: &[(String, String)]) -> String {
    let (mut good, mut checked) = (0usize, 0usize);
    for t in fns {
        let path = phi4.iter().find(|(f, p)| p.ends_with(&format!("{}.rs", t.name)) && *f == t.family);
        if let Some(code) = path.and_then(|(_, p)| std::fs::read_to_string(p).ok()) {
            if let Some(w) = works(t, &code) {
                good += w as usize;
                checked += 1;
                if std::env::var("CODE_WRITE_DEBUG").is_ok() {
                    eprintln!("teacher {} works={w}", t.name);
                }
            }
        }
    }
    format!("| **Phi-4 itself (the teacher)** | – | – | {good} of {checked} ({:.0}%) | – | 100% |\n", 100.0 * good as f64 / checked.max(1) as f64)
}
