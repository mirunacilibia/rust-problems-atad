use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'absolutePermutation' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER k
 */

fn absolutePermutation(n: i32, k: i32) -> Vec<i32> {
    if k == 0 {
        return (1..=n).collect();
    }

    if n % (2 * k) != 0 {
        return vec![-1];
    }

    let mut result = Vec::new();
    let mut add = true;

    for i in 1..=n {
        if add {
            result.push(i + k);
        } else {
            result.push(i - k);
        }

        if i % k == 0 {
            add = !add;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let result = absolutePermutation(n, k);

        for i in 0..result.len() {
            write!(&mut fptr, "{}", result[i]).ok();

            if i != result.len() - 1 {
                write!(&mut fptr, " ").ok();
            }
        }

        writeln!(&mut fptr).ok();
    }
}
