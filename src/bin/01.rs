use std::cmp;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/01.in").unwrap();
    let masses: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();

    let p1 = solve_p1(&masses);
    println!("Part One: {p1}");

    let p2 = solve_p2(&masses);
    println!("Part Two: {p2}");
}

fn solve_p1(masses: &[i64]) -> i64 {
    masses.iter().map(|&mass| fuel(mass)).sum()
}

fn solve_p2(masses: &[i64]) -> i64 {
    masses.iter().map(|&mass| fuel_recursive(mass)).sum()
}

fn fuel(mass: i64) -> i64 {
    cmp::max(mass / 3 - 2, 0)
}

fn fuel_recursive(mass: i64) -> i64 {
    let mut x = fuel(mass);
    if x > 0 {
        x += fuel_recursive(x);
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
        assert_eq!(fuel(-1), 0);
    }

    #[test]
    fn test_fuel_recursive() {
        assert_eq!(fuel_recursive(14), 2);
        assert_eq!(fuel_recursive(1969), 966);
        assert_eq!(fuel_recursive(100756), 50346);
        assert_eq!(fuel_recursive(-1), 0);
    }
}
