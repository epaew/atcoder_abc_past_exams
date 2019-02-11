fn read_single<T: std::str::FromStr>() -> T {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let h1: i32 = read_single();
    let h2: i32 = read_single();

    println!("{}", h1 - h2)
}
