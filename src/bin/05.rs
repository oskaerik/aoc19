use std::fs;

fn main() {
    let input = fs::read_to_string("input/05.in").unwrap();
    println!("{input}");

    let program: Vec<i64> = input
        .lines()
        .next()
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .unwrap();

    let mut state = program.clone();
    let mut outputs: Vec<i64> = Vec::new();
    run(&mut state, &[1], &mut outputs);
    assert!(outputs.iter().take(outputs.len() - 1).all(|&x| x == 0));
    let p1 = outputs.last().unwrap();
    println!("Part One: {p1}");

    state = program.clone();
    outputs = Vec::new();
    run(&mut state, &[5], &mut outputs);
    assert!(outputs.len() == 1);
    let p2 = outputs.first().unwrap();
    println!("Part Two: {p2}");
}

fn run<'a>(state: &'a mut [i64], inputs: &[i64], outputs: &mut Vec<i64>) -> &'a [i64] {
    let mut inputs_iter = inputs.iter();

    let mut i = 0;
    while state[i] != 99 {
        let mut n = state[i];
        let op = n % 100;
        n /= 100;
        let mode1 = n % 10;
        n /= 10;
        let mode2 = n % 10;
        n /= 10;
        let mode3 = n % 10;

        match op {
            1 => {
                assert!(mode3 == 0);
                let a = state[i + 1];
                let b = state[i + 2];
                let c = state[i + 3];
                state[c as usize] = if mode1 == 0 { state[a as usize] } else { a }
                    + if mode2 == 0 { state[b as usize] } else { b };
                i += 4;
            }
            2 => {
                assert!(mode3 == 0);
                let a = state[i + 1];
                let b = state[i + 2];
                let c = state[i + 3];
                state[c as usize] = if mode1 == 0 { state[a as usize] } else { a }
                    * if mode2 == 0 { state[b as usize] } else { b };
                i += 4;
            }
            3 => {
                assert!(mode1 == 0);
                let a = state[i + 1];
                state[a as usize] = *inputs_iter.next().unwrap();
                i += 2;
            }
            4 => {
                let a = state[i + 1];
                outputs.push(if mode1 == 0 { state[a as usize] } else { a });
                i += 2;
            }
            5 => {
                let a = state[i + 1];
                let b = state[i + 2];
                let jmp = if mode1 == 0 { state[a as usize] } else { a };
                if jmp != 0 {
                    i = (if mode2 == 0 { state[b as usize] } else { b })
                        .try_into()
                        .unwrap();
                } else {
                    i += 3;
                }
            }
            6 => {
                let a = state[i + 1];
                let b = state[i + 2];
                let jmp = if mode1 == 0 { state[a as usize] } else { a };
                if jmp == 0 {
                    i = (if mode2 == 0 { state[b as usize] } else { b })
                        .try_into()
                        .unwrap();
                } else {
                    i += 3;
                }
            }
            7 => {
                assert!(mode3 == 0);
                let a = state[i + 1];
                let b = state[i + 2];
                let c = state[i + 3];
                let lt = if mode1 == 0 { state[a as usize] } else { a }
                    < if mode2 == 0 { state[b as usize] } else { b };
                state[c as usize] = if lt { 1 } else { 0 };
                i += 4;
            }
            8 => {
                assert!(mode3 == 0);
                let a = state[i + 1];
                let b = state[i + 2];
                let c = state[i + 3];
                let eq = if mode1 == 0 { state[a as usize] } else { a }
                    == if mode2 == 0 { state[b as usize] } else { b };
                state[c as usize] = if eq { 1 } else { 0 };
                i += 4;
            }
            _ => unreachable!(),
        }
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let mut outputs: Vec<i64> = Vec::new();
        run(&mut [3, 0, 4, 0, 99], &[1], &mut outputs);
        assert_eq!(outputs, [1]);

        outputs = Vec::new();
        assert_eq!(
            run(&mut [1002, 4, 3, 4, 33], &[], &mut outputs),
            [1002, 4, 3, 4, 99]
        );

        outputs = Vec::new();
        run(
            &mut [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            &[8],
            &mut outputs,
        );
        assert_eq!(outputs, [1]);

        outputs = Vec::new();
        run(
            &mut [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            &[9],
            &mut outputs,
        );
        assert_eq!(outputs, [0]);

        outputs = Vec::new();
        run(
            &mut [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            &[7],
            &mut outputs,
        );
        assert_eq!(outputs, [1]);

        outputs = Vec::new();
        run(
            &mut [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            &[8],
            &mut outputs,
        );
        assert_eq!(outputs, [0]);

        outputs = Vec::new();
        run(&mut [3, 3, 1108, -1, 8, 3, 4, 3, 99], &[8], &mut outputs);
        assert_eq!(outputs, [1]);

        outputs = Vec::new();
        run(&mut [3, 3, 1108, -1, 8, 3, 4, 3, 99], &[9], &mut outputs);
        assert_eq!(outputs, [0]);

        outputs = Vec::new();
        run(&mut [3, 3, 1107, -1, 8, 3, 4, 3, 99], &[7], &mut outputs);
        assert_eq!(outputs, [1]);

        outputs = Vec::new();
        run(&mut [3, 3, 1107, -1, 8, 3, 4, 3, 99], &[8], &mut outputs);
        assert_eq!(outputs, [0]);

        outputs = Vec::new();
        run(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[0],
            &mut outputs,
        );
        assert_eq!(outputs, [0]);

        outputs = Vec::new();
        run(
            &mut [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[-1],
            &mut outputs,
        );
        assert_eq!(outputs, [1]);

        outputs = Vec::new();
        run(
            &mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &[0],
            &mut outputs,
        );
        assert_eq!(outputs, [0]);

        outputs = Vec::new();
        run(
            &mut [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            &[-1],
            &mut outputs,
        );
        assert_eq!(outputs, [1]);

        outputs = Vec::new();
        run(
            &mut [
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &[7],
            &mut outputs,
        );
        assert_eq!(outputs, [999]);

        outputs = Vec::new();
        run(
            &mut [
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &[8],
            &mut outputs,
        );
        assert_eq!(outputs, [1000]);

        outputs = Vec::new();
        run(
            &mut [
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            &[9],
            &mut outputs,
        );
        assert_eq!(outputs, [1001]);
    }
}
