use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn valid_parentheses(s: &str) -> bool {
    let (acc, error) = s
        .chars()
        .fold_while((0, false), |(acc, error), e| match e {
            '(' => Continue((acc + 1, false)),
            ')' if acc == 0 => Done((0, true)),
            ')' => Continue((acc - 1, false)),
            _ => return Continue((acc, error)),
        })
        .into_inner();
    if error || acc != 0 {
        false
    } else {
        true
    }
}

fn main() {
    println!("Hello, world!");
}
