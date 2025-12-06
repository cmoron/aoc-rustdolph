fn main() {
    let input = include_str!("../input.txt");

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example_input = include_str!("../example.txt");
        assert_eq!(part1(example_input), 0);
    }
}
