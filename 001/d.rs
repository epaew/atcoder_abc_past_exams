fn read_single<T: std::str::FromStr>() -> T {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.lock().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn read_matrix<T: std::str::FromStr>(n: i32) -> Vec<Vec<T>> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut v = Vec::new();
    for _ in 0..n {
        let mut buf = String::new();
        stdin.lock().read_line(&mut buf).unwrap();
        let v0 = buf
            .trim()
            .split('-')
            .map(|e| e.parse().ok().unwrap())
            .collect();
        v.push(v0)
    }
    v
}

fn main() {
    let n: i32 = read_single();
    let mut v: Vec<Vec<i32>> = read_matrix(n);
    v.sort_by(|l1, l2| l1.first().cmp(&l2.first()));

    let mut results: Vec<Vec<i32>> = Vec::new();
    for l in v {
        let start = l.first().unwrap() / 5 * 5;
        let mut end = ((l.last().unwrap() - 1) / 5 + 1) * 5;
        if end % 100 == 60 {
            end += 40
        }
        let converted = [start, end].to_vec();

        if results.is_empty() {
            results.push(converted);
        } else if results.last().unwrap().last() < converted.first() {
            results.push(converted);
        } else {
            if results.last().unwrap().last() < converted.last() {
                let mut last = results.pop().unwrap();
                last.pop();
                last.push(*converted.last().unwrap());
                results.push(last);
            }
        }
    }

    for l in results {
        println!("{:04}-{:04}", l.first().unwrap(), l.last().unwrap())
    }
}
