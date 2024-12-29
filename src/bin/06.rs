#![warn(clippy::pedantic)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    let input = fs::read_to_string("input/06.in").unwrap();
    let (p1, p2) = solve(&input);
    println!("Part One: {p1}");
    println!("Part Two: {p2}");
}

fn solve(input: &str) -> (i64, i64) {
    let (orbited, orbits) = parse(input);
    println!("{orbits:?}");

    let mut p1 = 0;

    let mut queue = VecDeque::new();
    queue.push_back(("COM", 0));

    let mut visited = HashSet::new();
    visited.insert("COM");

    while !queue.is_empty() {
        let (node, depth) = queue.pop_front().unwrap();
        p1 += depth;
        if !orbited.contains_key(node) {
            continue;
        }
        for neighbor in &orbited[node] {
            if visited.contains(neighbor) {
                continue;
            }
            queue.push_back((neighbor, depth + 1));
            visited.insert(neighbor);
        }
    }

    if !orbits.contains_key("YOU") {
        return (p1, 0);
    }

    let mut p2 = 0;

    let mut queue = VecDeque::new();
    queue.push_back((orbits["YOU"].first().unwrap(), 0));

    let mut visited = HashSet::new();
    visited.insert(orbits["YOU"].first().unwrap());

    'outer: while !queue.is_empty() {
        let (node, depth) = queue.pop_front().unwrap();
        if orbited.contains_key(node) {
            for neighbor in &orbited[node] {
                if visited.contains(neighbor) {
                    continue;
                }
                if neighbor == orbits["SAN"].first().unwrap() {
                    p2 = depth + 1;
                    break 'outer;
                }
                queue.push_back((neighbor, depth + 1));
                visited.insert(neighbor);
            }
        }
        if orbits.contains_key(node) {
            for neighbor in &orbits[node] {
                if visited.contains(neighbor) {
                    continue;
                }
                if neighbor == orbits["SAN"].first().unwrap() {
                    p2 = depth + 1;
                    break 'outer;
                }
                queue.push_back((neighbor, depth + 1));
                visited.insert(neighbor);
            }
        }
    }

    (p1, p2)
}

fn parse(input: &str) -> (Graph, Graph) {
    let mut orbited = HashMap::new();
    let mut orbits = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once(')').unwrap();
        orbited.entry(a).or_insert_with(Vec::new);
        orbited.get_mut(a).unwrap().push(b);
        orbits.entry(b).or_insert_with(Vec::new);
        orbits.get_mut(b).unwrap().push(a);
    }
    (orbited, orbits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve(
                "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
            )
            .0,
            42
        );
        assert_eq!(
            solve(
                "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
            )
            .1,
            4
        );
    }
}
