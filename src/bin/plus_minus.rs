use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let (mut pos , mut neg , mut nut) = (0 , 0 , 0);
    for el in arr.iter() {
        match *el {
            x if x > 0 => pos += 1,
            x if x < 0 => neg += 1,
            _ => nut += 1
        };
    }
    println!("{:.6}", pos as f32 / arr.len() as f32 );
    println!("{:.6}", neg as f32 / arr.len() as f32 );
    println!("{:.6}", nut as f32 / arr.len() as f32 );
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
