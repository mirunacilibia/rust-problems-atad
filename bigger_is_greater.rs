use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'biggerIsGreater' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING w as parameter.
 */

fn biggerIsGreater(w: &str) -> String {
    let mut chars: Vec<char> = w.chars().collect();
    let len = chars.len();

    // Step 1: Find the pivot, the first character from the end that is smaller than the one after it
    let mut i = len - 1;
    while i > 0 && chars[i - 1] >= chars[i] {
        i -= 1;
    }

    if i == 0 {
        return "no answer".to_string(); // No greater word possible
    }

    // Step 2: Find the smallest character on the right of pivot that is greater than the pivot
    let pivot = i - 1;
    let mut j = len - 1;
    while chars[j] <= chars[pivot] {
        j -= 1;
    }

    // Step 3: Swap the pivot with this character
    chars.swap(pivot, j);

    // Step 4: Reverse the suffix starting from pivot + 1
    chars[pivot + 1..].reverse();

    chars.into_iter().collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let T = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..T {
        let w = stdin_iterator.next().unwrap().unwrap();

        let result = biggerIsGreater(&w);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
