use std::fs;

fn main() {
    let input = fs::read_to_string("input/04.in").unwrap();

    let (a, b) = input
        .lines()
        .next()
        .and_then(|line| line.split_once('-'))
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .unwrap();

    let mut p1: i64 = 0;
    let mut p2: i64 = 0;
    for x in a..b + 1 {
        if part1(x) {
            p1 += 1
        };
        if part2(x) {
            p2 += 1
        };
    }
    println!("Part One: {p1}");
    println!("Part Two: {p2}");
}

fn part1(x: i64) -> bool {
    let mut n = x;
    let mut d = 10;
    let mut inc = true;
    let mut dub = false;
    while n > 0 {
        let d_ = n % 10;
        inc = inc && d_ <= d;
        dub = dub || d == d_;

        d = d_;
        n /= 10;
    }
    inc && dub
}

fn part2(x: i64) -> bool {
    let mut n = x;
    let mut d = 10;
    let mut c = 1;
    let mut inc = true;
    let mut dub = false;
    while n > 0 {
        let d_ = n % 10;

        if d == d_ {
            c += 1;
        } else {
            if c == 2 {
                dub = true;
            }
            c = 1;
        }

        inc = inc && d_ <= d;

        d = d_;
        n /= 10;
    }
    if c == 2 {
        dub = true;
    }
    inc && dub
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(part1(111111));
        assert!(!part1(223450));
        assert!(!part1(123789));
    }

    #[test]
    fn test_part2() {
        assert!(part2(112233));
        assert!(!part2(123444));
        assert!(!part2(1234445));
        assert!(part2(111122));
        assert!(part2(1111223));
        assert!(!part2(1111221));
        assert!(!part2(123));
        assert!(!part2(2111223));
        assert!(!part2(211122));
        assert!(!part2(1));
        assert!(part2(11));
    }
}
