use std::collections::HashMap;

pub fn solve(input: &str) -> i32 {
    let (l1, l2) = lines_to_vectors(input);

    l1.iter().zip(l2.iter()).map(|(&a, &b)| (a - b).abs()).sum()
}

pub fn solve2(input: &str) -> i32 {
    let (l1, l2) = lines_to_vectors(input);

    let l2_hash: HashMap<i32, i32> = l2.into_iter().fold(HashMap::new(), |mut map, n| {
        *map.entry(n).or_insert(0) += 1;
        map
    });

    l1.into_iter()
        .map(|n| n * l2_hash.get(&n).copied().unwrap_or(0))
        .sum()
}

fn lines_to_vectors(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();

    for line in input.lines() {
        let mut items = line.split("   ");
        if let (Some(first), Some(second)) = (items.next(), items.next()) {
            if let (Ok(first_num), Ok(second_num)) = (first.parse::<i32>(), second.parse::<i32>()) {
                l1.push(first_num);
                l2.push(second_num);
            }
        }
    }

    l1.sort_unstable();
    l2.sort_unstable();

    (l1, l2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample() {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(solve(input), 11);
    }

    #[test]
    fn solve2_sample() {
        let input = "\
3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(solve2(input), 31);
    }
}
