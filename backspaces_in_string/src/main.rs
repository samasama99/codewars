fn clean_string(s: &str) -> String {
    s.chars().fold(String::new(), |mut acc, c| {
        if c == '#' {
            acc.pop();
        } else {
            acc.push(c);
        }
        acc
    })
}

fn main() {
    println!("Hello, world!");
}
