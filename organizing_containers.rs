use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'organizingContainers' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts 2D_INTEGER_ARRAY container as parameter.
 */

fn organizingContainers(container: &[Vec<i32>]) -> String {
    let n = container.len();

    let mut row_sums: Vec<i32> = vec![0; n];
    let mut col_sums: Vec<i32> = vec![0; n];

    for i in 0..n {
        for j in 0..n {
            row_sums[i] += container[i][j];
            col_sums[j] += container[i][j];
        }
    }

    row_sums.sort_unstable();
    col_sums.sort_unstable();

    if row_sums == col_sums {
        "Possible".to_string()
    } else {
        "Impossible".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let mut container: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

        for i in 0..n as usize {
            container.push(Vec::with_capacity(n as usize));

            container[i] = stdin_iterator.next().unwrap().unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect();
        }

        let result = organizingContainers(&container);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
