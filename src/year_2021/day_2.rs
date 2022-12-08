#[aoc(day2, part1)]
fn part_1(input: &str) -> usize {
    let mut depth = 0;
    let mut horizontal = 0;
    for line in input.lines() {
        let [cmd, amount] = line.split_whitespace().collect::<Vec<_>>()[..] else { unreachable!() };
        let amount = amount.parse::<usize>().unwrap();
        match cmd {
            "forward" => {
                horizontal += amount;
            }
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => {
                unreachable!()
            }
        }
    }
    depth * horizontal
}

#[aoc(day2, part2)]
fn part_2(input: &str) -> usize {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for line in input.lines() {
        let [cmd, amount] = line.split_whitespace().collect::<Vec<_>>()[..] else { unreachable!() };
        let amount = amount.parse::<usize>().unwrap();
        match cmd {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => {
                unreachable!()
            }
        }
    }
    depth * horizontal
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT), 150)
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT), 900)
    }
}
