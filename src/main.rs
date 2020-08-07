extern crate huffman;

use huffman::huffman::*;
use std::env;

fn make_list(n: usize, p: f64) -> Vec<(String, f64)> {
    let mut res = Vec::new();
    rec(&mut res, String::new(), n, 1f64, p);
    res
}

fn rec(list: &mut Vec<(String, f64)>, code: String, n: usize, acc: f64, p: f64) {
    if n == 0 {
        list.push((code, acc));
        return;
    }

    rec(list, format!("{}0", &code), n - 1, acc * p, p);
    rec(list, format!("{}1", &code), n - 1, acc * (1f64 - p), p);
}

fn main() {
    let mut args = env::args();
    args.next();

    let n = match args.next() {
        Some(s) => s.parse::<usize>().unwrap_or(1),
        None => 1,
    };

    let p = match args.next() {
        Some(s) => s.parse::<f64>().unwrap_or(0.5),
        None => 0.5,
    };

    let list = make_list(n, p);

    let entropy = list.iter().map(|(_, v)| v * (-v.log2())).sum::<f64>();
    let huf_code = HuffmanCode::new(list).unwrap();

    println!(
        "entropy: {}\n{}\n{}\navg_len: {}",
        entropy,
        huf_code.get_tree(),
        huf_code,
        huf_code.get_avg_len()
    );
}
