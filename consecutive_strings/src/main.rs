use std::cmp::Ordering;

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let array_len = strarr.len();
    if array_len == 0 || k > array_len || k == 0 {
        return String::from("");
    }
    strarr
        .windows(k)
        .map(|s| s.join(""))
        .max_by(|s1, s2| s1.len().cmp(&s2.len()).then_with(|| Ordering::Less))
        .unwrap()
}

fn main() {
    println!(
        "{}",
        longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3)
    );
}


70403
