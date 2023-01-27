use std::collections::HashMap;

fn number_of_pairs(gloves: &[&str]) -> u32 {
    let mut map = HashMap::new();
    gloves.iter().fold(0, |acc, e| match map.get_mut(e) {
        Some(val) if *val => {
            *val = false;
            acc + 1
        }
        Some(val) => {
            *val = true;
            acc
        }
        None => {
            map.insert(e, true);
            acc
        }
    })
}

fn main() {
    println!("Hello, world!");
}
