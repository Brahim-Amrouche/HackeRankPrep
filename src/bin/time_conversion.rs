use core::time;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

 fn time_string(time: u32) -> String
 {
     match time {
         t if t < 10 => "0".to_string() + &time.to_string(),
         _ => time.to_string()
     }
 }
 
 
 fn timeConversion(s: &str) -> String {
     let pure_s = &s[0..8];
     let mut time_parts:Vec<u32> = pure_s.split(':').map(|c| c.parse::<u32>().unwrap()).collect();
     let is_morning = s.chars().nth(8).unwrap() == 'A';
     if !is_morning
     {
         if  time_parts[0] < 12
         {
             time_parts[0] = 12 + time_parts[0];
         }
     }
     else if is_morning && time_parts[0] == 12
     {
         time_parts[0] = 0
     }
     let military_time = time_string(time_parts[0]) + &":" + &time_string(time_parts[1]) + &":" + &time_string(time_parts[2]);
     military_time
 }

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
