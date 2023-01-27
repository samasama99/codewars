fn expanded_form(n: u64) -> String {
    let num = n.to_string();
    let len = num.len();
    num.chars()
        .enumerate()
        .filter(|(_, c)| *c != '0')
        .map(|(index, c)| c.to_string() + &"0".repeat(len - index))
        .reduce(|n1, n2| n1 + " + " + &n2)
        .unwrap_or(String::from(""))
        .to_owned()
}

fn main() {
    println!("{}", expanded_form(70304));
}
