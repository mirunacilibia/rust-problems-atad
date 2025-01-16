use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'encryption' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn encryption(s: &str) -> String {
    let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    let len = cleaned.len();
    let rows = (len as f64).sqrt().floor() as usize;
    let cols = (len as f64).sqrt().ceil() as usize;

    let mut grid = vec![String::new(); cols];

    for (i, ch) in cleaned.chars().enumerate() {
        grid[i % cols].push(ch);
    }

    grid.join(" ")
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = encryption(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
