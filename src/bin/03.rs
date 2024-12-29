use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/03.in").unwrap();
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let second = lines.next().unwrap();

    let p1 = intersect(first, second);
    println!("Part One: {p1}");

    let p2 = steps(first, second);
    println!("Part Two: {p2}");
}

fn intersect(first: &str, second: &str) -> i64 {
    let path1: HashSet<_> = path(first).into_iter().collect();
    let path2: HashSet<_> = path(second).into_iter().collect();
    path1
        .intersection(&path2)
        .map(|(y, x)| y.abs() + x.abs())
        .min()
        .unwrap()
}

fn steps(first: &str, second: &str) -> i64 {
    let path1 = path(first);
    let path2 = path(second);

    let path1_set: HashSet<_> = path(first).iter().cloned().collect();
    let path2_set: HashSet<_> = path(second).iter().cloned().collect();

    path1_set
        .intersection(&path2_set)
        .map(|&t| {
            path1.iter().position(|&x| x == t).unwrap()
                + path2.iter().position(|&x| x == t).unwrap()
                + 2
        })
        .min()
        .unwrap()
        .try_into()
        .unwrap()
}

fn path(traces: &str) -> Vec<(i64, i64)> {
    let mut pos = (0, 0);
    let mut p = Vec::new();
    for trace in traces.split(",") {
        let n: i64 = trace[1..].parse().unwrap();
        match trace.chars().next() {
            Some('U') => {
                for _ in 0..n {
                    pos = (pos.0 - 1, pos.1);
                    p.push(pos);
                }
            }
            Some('D') => {
                for _ in 0..n {
                    pos = (pos.0 + 1, pos.1);
                    p.push(pos);
                }
            }
            Some('L') => {
                for _ in 0..n {
                    pos = (pos.0, pos.1 - 1);
                    p.push(pos);
                }
            }
            Some('R') => {
                for _ in 0..n {
                    pos = (pos.0, pos.1 + 1);
                    p.push(pos);
                }
            }
            _ => unreachable!(),
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        assert_eq!(intersect("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
        assert_eq!(
            intersect(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
        assert_eq!(
            intersect(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn test_steps() {
        assert_eq!(steps("R8,U5,L5,D3", "U7,R6,D4,L4"), 30);
        assert_eq!(
            steps(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            610
        );
        assert_eq!(
            steps(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
