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
    let deg = inputs[0];
    let dis = inputs[1];

    let w = match (dis as f32 / 6.0).round() / 10.0 {
        0.0...0.2 => 0,
        0.3...1.5 => 1,
        1.6...3.3 => 2,
        3.4...5.4 => 3,
        5.5...7.9 => 4,
        8.0...10.7 => 5,
        10.8...13.8=> 6,
        13.9...17.1 => 7,
        17.2...20.7 => 8,
        20.8...24.4 => 9,
        24.5...28.4 => 10,
        28.5...32.6 => 11,
        32.7...std::f32::INFINITY => 12,
        _ => 0
    };
    let dir = if w == 0 {
        "C"
    } else {
        match deg * 10 {
            1125...3375 => "NNE",
            3375...5625 => "NE",
            5625...7875 => "ENE",
            7875...10125 => "E",
            10125...12375 => "ESE",
            12375...14625 => "SE",
            14625...16875 => "SSE",
            16875...19125 => "S",
            19125...21375 => "SSW",
            21375...23625 => "SW",
            23625...25875 => "WSW",
            25875...28125 => "W",
            28125...30375 => "WNW",
            30375...32625 => "NW",
            32625...34875 => "NNW",
            _ => "N",
        }
    };
    println!("{} {}", dir ,w);
}
