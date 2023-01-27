fn fold_array(arr: &[i32], runs: usize) -> Vec<i32> {
    (0..runs).fold(arr.to_vec(), |acc, _| {
        acc.iter()
            .enumerate()
            .zip(acc.iter().enumerate().rev())
            .map_while(|((index1, e1), (index2, e2))| {
                if index1 > index2 {
                    return None;
                }
                Some(e1 + if index1 == index2 { 0 } else { *e2 })
            })
            .collect()
    })
}

fn main() {
    dbg!(fold_array(&[1, 2, 3, 4, 5], 0));
    dbg!(fold_array(&[1, 2, 3, 4, 5], 1));
    dbg!(fold_array(&[1, 2, 3, 4, 5], 2));
    dbg!(fold_array(&[1, 2, 3, 4, 5], 3));
}
