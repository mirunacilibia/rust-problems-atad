use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'climbingLeaderboard' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY ranked
 *  2. INTEGER_ARRAY player
 */

fn climbingLeaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let mut ranks = Vec::new();
    let mut distinct_ranked: Vec<i32> = ranked.iter().cloned().collect();
    distinct_ranked.dedup();

    let mut index = distinct_ranked.len() as i32 - 1;

    for &score in player {
        while index >= 0 && score >= distinct_ranked[index as usize] {
            index -= 1;
        }
        ranks.push(index + 2);
    }

    ranks
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ranked_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ranked: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let _player_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let player: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = climbingLeaderboard(&ranked, &player);

    for i in 0..result.len() {
        writeln!(&mut fptr, "{}", result[i]).ok();
    }
}
