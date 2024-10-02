use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/*
 * Complete the 'lonelyinteger' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn lonelyinteger(a: &[i32]) -> i32 {
    let mut occuring_elements:HashMap<i32, bool> = HashMap::new();
    let mut res= 0;
    for el in a
    {
        match occuring_elements.get_mut(el)
        {
            Some(v) => {
                *v = true;
            },
            None => 
            {
                occuring_elements.insert(*el, false);
            }
        }
    };
    for (key, val) in occuring_elements
    {
        if val == false
        {
            res = key;
            break;
        }
    }
    return  res;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = lonelyinteger(&a);

    writeln!(&mut fptr, "{}", result).ok();
}