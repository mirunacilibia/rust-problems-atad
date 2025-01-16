# Problem Summaries

I solved 6 easy problems and 6 medium problems (1 + 0.5 * 6 + 1 * 6 = 10)

## 1. Solve Me First (Difficulty: easy)
**Task**:  
The task is to calculate the sum of 2 integers.

**Solution**:  
I simply returned the sum of the 2 variables.

## 2. Simple Array Sum (Difficulty: easy)
**Task**:  
The task is to calculate the sum of an array of integers.

**Solution**:  
I used the `.iter().sum()` method to calculate the sum.

**Code**:
```rust
fn simpleArraySum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}
```
## 3. A Very Big Sum (Difficulty: easy)
**Task**:  
The task is to compute the sum of an array of integers where the values can be very large.

**Solution**:  
I used the `.iter().sum()` method to calculate the sum, ensuring it works with `i64` for large values.

**Code**:
```rust
fn aVeryBigSum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}
```

## 4. Diagonal Difference (Difficulty: easy)
**Task**:  
Given a square matrix, calculate the absolute difference between the sums of its two diagonals.

**Solution**:  
I iterated through the matrix, summing up the primary and secondary diagonal values, and then calculated their absolute difference using `.abs()`.

**Code**:
```rust
fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diagonal = 0;
    let mut secondary_diagonal = 0;

    for i in 0..n {
        primary_diagonal += arr[i][i];
        secondary_diagonal += arr[i][n - 1 - i];
    }

    (primary_diagonal - secondary_diagonal).abs()
}
```

## 5. Plus Minus (Difficulty: easy)
**Task**:  
Given an array of integers, calculate the ratio of positive, negative, and zero elements, and print each ratio with six decimal places.

**Solution**:  
I used `.filter()` to count positive, negative, and zero elements. Then, I calculated their ratios by dividing the counts by the total number of elements and formatted the output to six decimal places using `println!`.

**Code**:
```rust
fn plusMinus(arr: &[i32]) {
    let n = arr.len() as f64;
    let positive_count = arr.iter().filter(|&&x| x > 0).count() as f64;
    let negative_count = arr.iter().filter(|&&x| x < 0).count() as f64;
    let zero_count = arr.iter().filter(|&&x| x == 0).count() as f64;

    println!("{:.6}", positive_count / n);
    println!("{:.6}", negative_count / n);
    println!("{:.6}", zero_count / n);
}
```
## 6. Staircase (Difficulty: easy)
**Task**:  
Print a staircase of size `n` using `#` symbols and spaces, where the last line has no leading spaces.

**Solution**:  
I used a loop to print `n` rows, with spaces on the left and `#` symbols on the right for each row. The number of spaces decreases as the number of `#` symbols increases.

**Code**:
```rust
fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        println!("{}{}", spaces, hashes);
    }
}

```

## 7. Climbing the Leaderboard (Difficulty: medium)
**Task**:  
Find a player's rank in a dense ranking system using a list of existing scores and new scores.

**Solution**:  
I deduplicated the scores using `.dedup()`, iterated through the player's scores, and compared them with the ranked list to assign the appropriate rank. The rank was calculated by keeping track of the player's position relative to the existing scores.

**Code**:
```rust
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
```

## 8. Organizing Containers of Balls (Difficulty: medium)
**Task**:  
Determine if it is possible to organize balls such that each container holds only one type of ball.

**Solution**:  
I calculated the total capacity of each container and the total count of each ball type. If these match (after sorting), the solution is `"Possible"`. Otherwise, it is `"Impossible"`.

**Code**:
```rust
fn organizingContainers(container: &[Vec<i32>]) -> String {
    let container_capacities: Vec<i32> = container.iter().map(|row| row.iter().sum()).collect();
    let ball_type_counts: Vec<i32> = (0..container.len())
        .map(|i| container.iter().map(|row| row[i]).sum())
        .collect();

    let mut capacities_sorted = container_capacities;
    let mut counts_sorted = ball_type_counts;
    capacities_sorted.sort();
    counts_sorted.sort();

    if capacities_sorted == counts_sorted {
        "Possible".to_string()
    } else {
        "Impossible".to_string()
    }
}
```

## 9. Encryption (Difficulty: medium)
**Task**:  
Encrypt a given text by removing spaces, placing characters in a grid, and reading the columns of the grid to produce the encrypted text.

**Solution**:  
I removed spaces from the text, calculated the grid size, and then iterated through the columns to build the encrypted text.

**Code**:
```rust
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
```

## 10. Bigger is Greater (Difficulty: medium)
**Task**:  
Find the lexicographically smallest permutation that is greater than a given word, or return `"no answer"` if it doesn't exist.

**Solution**:  
I identified the pivot where the word can be rearranged, swapped it with the smallest larger character, and reversed the suffix to form the next lexicographical permutation.

**Code**:
```rust
fn biggerIsGreater(w: &str) -> String {
    let mut chars: Vec<char> = w.chars().collect();
    let len = chars.len();

    let mut i = len - 1;
    while i > 0 && chars[i - 1] >= chars[i] {
        i -= 1;
    }

    if i == 0 {
        return "no answer".to_string();
    }

    let pivot = i - 1;
    let mut j = len - 1;
    while chars[j] <= chars[pivot] {
        j -= 1;
    }

    chars.swap(pivot, j);
    chars[pivot + 1..].reverse();

    chars.into_iter().collect()
}
```

## 11. The Time in Words (Difficulty: medium)
**Task**:  
Convert a given time (hour and minutes) into a textual representation in words.

**Solution**:  
I used predefined word mappings for numbers and handled special cases like `"quarter"` and `"half"`. Depending on whether the minutes are past or to the hour, I constructed the appropriate text format.

**Code**:
```rust
fn timeInWords(h: i32, m: i32) -> String {
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve",
        "thirteen", "fourteen", "quarter", "sixteen", "seventeen", "eighteen", "nineteen", "twenty", "twenty one",
        "twenty two", "twenty three", "twenty four", "twenty five", "twenty six", "twenty seven", "twenty eight", "twenty nine", "half"
    ];

    match m {
        0 => format!("{} o' clock", numbers[h as usize]),
        1 => format!("one minute past {}", numbers[h as usize]),
        15 => format!("quarter past {}", numbers[h as usize]),
        30 => format!("half past {}", numbers[h as usize]),
        45 => format!("quarter to {}", numbers[(h % 12 + 1) as usize]),
        1..=29 => format!("{} minutes past {}", numbers[m as usize], numbers[h as usize]),
        31..=59 => format!("{} minutes to {}", numbers[60 - m as usize], numbers[(h % 12 + 1) as usize]),
        _ => String::new(),
    }
}
```

## 12. Absolute Permutation (Difficulty: medium)
**Task**:  
Construct an absolute permutation for a given `n` and `k` such that \(|\text{pos}[i] - i| = k\), or return `-1` if it's not possible.

**Solution**:  
I checked if the permutation was valid by ensuring `n % (2 * k) == 0`. If valid, I alternated between adding and subtracting `k` to construct the permutation. If the condition wasn't met, I returned `-1`.

**Code**:
```rust
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
```
