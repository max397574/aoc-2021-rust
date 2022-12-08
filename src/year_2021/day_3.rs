#[aoc(day3, part1)]
fn part_1(input: &str) -> usize {
    let mut bit_1: Vec<usize> = vec![0; input.lines().next().unwrap().len()];
    let mut total = 0;
    for line in input.lines() {
        let bytes = line.bytes();
        for (index, byte) in bytes.enumerate() {
            if byte as char == '1' {
                bit_1[index] += 1;
            }
        }
        total += 1;
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for bit in bit_1.iter() {
        if bit > &(total / 2) {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma = usize::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon = usize::from_str_radix(epsilon.as_str(), 2).unwrap();
    gamma * epsilon
}

#[aoc(day3, part2)]
fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn part1() {
        assert_eq!(part_1(INPUT), 198)
    }

    #[test]
    fn part2() {
        assert_eq!(part_2(INPUT), 0)
    }
}
