use std::io::{self, BufRead};
use std::collections::BinaryHeap;
use std::cmp::Reverse;
/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let mut max_heap:BinaryHeap<i32> = BinaryHeap::new();
    let mut min:i64 = 0;
    let mut max:i64 = 0;
    for el in arr {
        max_heap.push(*el);
    }
    for idx in 0..arr.len() {
        let val = max_heap.pop().unwrap();
        max += if idx < arr.len() - 1 { val as i64 } else {0};
        min += if idx > 0 { val as i64 } else {0} ;
    }
    println!("{} {}", min, max);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
