use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/*
 * Complete the 'matchingStrings' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. STRING_ARRAY strings
 *  2. STRING_ARRAY queries
 */

fn matchingStrings(strings: &[String], queries: &[String]) -> Vec<i32> {
    let mut res : Vec<i32> = Vec::new();
    let mut queries_hash:HashMap<String, i32> = HashMap::new();
    for q in queries
    {
        if !queries_hash.contains_key(q)
        {
            queries_hash.insert(q.to_string(), 0);
        }
    }
    for s in strings
    {
        if queries_hash.contains_key(s)
        {
            *queries_hash.get_mut(s).unwrap() += 1;
        }
    }
    res.reserve(queries_hash.len());
    for q in queries
    {
        res.push(*queries_hash.get(q).unwrap());
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let strings_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut strings: Vec<String> = Vec::with_capacity(strings_count as usize);

    for _ in 0..strings_count {
        let strings_item = stdin_iterator.next().unwrap().unwrap();
        strings.push(strings_item);
    }

    let queries_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut queries: Vec<String> = Vec::with_capacity(queries_count as usize);

    for _ in 0..queries_count {
        let queries_item = stdin_iterator.next().unwrap().unwrap();
        queries.push(queries_item);
    }

    let res = matchingStrings(&strings, &queries);

    for i in 0..res.len() {
        write!(&mut fptr, "{}", res[i]).ok();

        if i != res.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
