fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_line(&mut buf).unwrap();
    buf.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    let inputs: Vec<i32> = read_vec();
    println!("{}", inputs.iter().max().unwrap())
}
