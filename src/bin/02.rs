use std::fs;

fn main() {
    let input = fs::read_to_string("input/02.in").unwrap();
    let program: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let p1 = solve_p1(&program);
    println!("Part One: {p1}");

    let p2 = solve_p2(&program);
    println!("Part Two: {p2}");
}

fn solve_p1(program: &[i64]) -> i64 {
    let mut state = program.to_owned();
    state[1] = 12;
    state[2] = 2;
    run(&mut state)[0]
}

fn solve_p2(program: &[i64]) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut state = program.to_owned();
            state[1] = noun;
            state[2] = verb;
            let output = run(&mut state)[0];
            if output == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!();
}

fn run(state: &mut [i64]) -> &[i64] {
    let mut i = 0;
    while state[i] != 99 {
        let a = state[i + 1] as usize;
        let b = state[i + 2] as usize;
        let c = state[i + 3] as usize;
        match state[i] {
            1 => state[c] = state[a] + state[b],
            2 => state[c] = state[a] * state[b],
            _ => unreachable!(),
        }
        i += 4;
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(
            run(&mut [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(run(&mut [1, 0, 0, 0, 99]), [2, 0, 0, 0, 99]);
        assert_eq!(run(&mut [2, 3, 0, 3, 99]), [2, 3, 0, 6, 99]);
        assert_eq!(run(&mut [2, 4, 4, 5, 99, 0]), [2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            run(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99]),
            [30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
