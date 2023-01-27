fn parts_sums(ls: &[u64]) -> Vec<u64> {
    let array_sum: u64 = ls.iter().sum();
    let partial_sums = ls.iter().scan(0, |acc, n| Some(*acc + n));

    let mut parts_sums = partial_sums.fold(vec![array_sum], |mut acc, current_sum| {
        acc.push(array_sum - current_sum);
        acc
    });
    parts_sums.push(0);
    parts_sums
}

fn main() {
    let v = vec![0, 1, 3, 6, 10];
    // dbg!(v
    //     .iter()
    //     .scan(0, |acc, n| Some(*acc + n))
    //     .collect::<Vec<_>>());
    dbg!(parts_sums(&v));
}
