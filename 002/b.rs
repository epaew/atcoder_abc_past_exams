fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_line(&mut buf).unwrap();
    let result = buf.trim().chars()
        .filter(|c| !['a', 'i', 'u', 'e', 'o'].contains(c))
        .collect::<String>();

    println!("{}", result)
}
