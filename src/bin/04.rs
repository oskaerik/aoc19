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
    for x in a..b + 1 {
        if password(x) {p1+=1};
    }
    println!("Part One: {p1}");
}

fn password(x: i64) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password() {
        assert!(password(111111));
        assert!(!password(223450));
        assert!(!password(123789));
    }
}
