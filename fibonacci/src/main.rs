fn main() {
    calculate_fibonacci(10);
}

fn calculate_fibonacci(n: usize) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    let mut vec1 = vec![0, 1];

    while vec1.len() <= n {
        vec1.push(vec1[vec1.len() - 1] + vec1[vec1.len() - 2]);
    }

    println!("{}", vec1[n]);
    vec1[n]
}

