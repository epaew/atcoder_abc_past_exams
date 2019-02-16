fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_line(&mut buf).unwrap();
    buf.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main() {
    let values: Vec<i32> = read_vec();
    let fs = values.into_iter().map(|i| i as f64).collect::<Vec<f64>>();
    let xd = (fs[2] - fs[0]) * (fs[5] - fs[1]);
    let yd = (fs[3] - fs[1]) * (fs[4] - fs[0]);
    println!("{:.1}", (xd - yd).abs() / 2.0)
}
