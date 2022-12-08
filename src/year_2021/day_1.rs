// #[aoc_generator(day1)]
// fn generator(input: &str) -> _ {}

#[aoc(day1, part1)]
fn part_1(input: &str) -> usize {
    let lines = input.split('\n');
    let mut increments = 0;
    let mut previous = 10000000;
    for line in lines {
        let val = line.parse().unwrap();
        if val > previous {
            increments += 1;
        }
        previous = val;
    }
    increments
}

#[aoc(day1, part2)]
fn part_2(input: &str) -> usize {
    let mut increments = 0;
    let mut previous = 65000;
    for line in input.split('\n').collect::<Vec<&str>>()[..].windows(3) {
        let val = line.iter().map(|f| f.parse::<u16>().unwrap()).sum();
        if val > previous {
            increments += 1;
        }
        previous = val;
    }
    increments
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT), 7);
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT), 5);
    }
}
