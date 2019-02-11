fn read_single<T: std::str::FromStr>() -> T {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let m: i32 = read_single();
    let result = match m {
        0...99 => "00".to_string(),
        100...5000 => format!("{:02}", m / 100),
        6000...30000 => format!("{:02}", m / 1000 + 50),
        35000...70000 => format!("{:02}", (m / 1000 - 30) / 5 + 80),
        70001...100000 => "89".to_string(),
        _ => "".to_string(),
    };
    println!("{}", result)
}
