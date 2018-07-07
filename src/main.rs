use std::io::prelude::*;
use std::io;
use std::ops::Range;


fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut cases = String::new();
    stdin.read_line(&mut cases).unwrap();
    let cases: usize = cases.trim_right().parse().unwrap();
    for case in 0..cases {
        let mut info = String::new();
        stdin.read_line(&mut info).unwrap();
        let info = info.trim_right();
        let mut info_iter = info.split(' ');
        let num_of_attractions: i64 = info_iter.next().unwrap().parse().unwrap();
        let k_attractions_per_visit: i64 = info_iter.next().unwrap().parse().unwrap();
        let visit: i64 = info_iter.next().unwrap().parse().unwrap();
        let next = next_attraction(num_of_attractions, k_attractions_per_visit, visit);
        let mut attraction_names: Vec<String> = Vec::with_capacity(num_of_attractions as usize);
        for attraction in 0.. num_of_attractions {
            let mut attraction_name = String::new();
            stdin.read_line(&mut attraction_name).unwrap();
            attraction_names.push(attraction_name.trim_right().to_string());
        }
        let result_indices = attraction_nums(num_of_attractions, k_attractions_per_visit, next);
        let result;
        if let Some(first_indices) = result_indices.0 {
            result = format!("{} {}", attraction_names[first_indices].join(" "), attraction_names[result_indices.1].join(" "));
        } else {
            result = attraction_names[result_indices.1].join(" ");
        }
        println!("Case #{}: {}", case + 1, result);
    }
}

// Number of the most popular attraction with the least visits
// NOT necessarily the first to be printed in output, as that is in popularity order.
fn next_attraction(n: i64, k: i64, v: i64) -> i64 {
    // Imagine the attractions arranged in a clock. The attractions are always visited in the same order and wrapping around, so we want modulo
    // Since v is the upcoming visit, we only want to compute with prior visits, so subtract 1.
    (k * (v-1)) % n
}

// Given n = a number of attractions, k = number of attractions in visit, and a = starting attraction, returns one or two ranges representing the attractions to visit in order
fn attraction_nums(n: i64, k: i64, a: i64) -> (Option<Range<usize>>, Range<usize>) {
    let n: usize = n as usize;
    let k: usize = k as usize;
    let a: usize = a as usize;
    let wrap = k+a > n; // If equal, first range is empty. Avoid unnecessary spaces
    let first = if wrap {
        Some(0..(k+a)-n)
    } else {
        None
    };
    let second = if wrap {
        a .. n
    } else {
        a .. (k+a)
    };
    (first, second)
}