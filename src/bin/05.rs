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
    let inputs: Vec<i64> = vec![1];
    let mut outputs: Vec<i64> = Vec::new();
    run(&mut state, &inputs, &mut outputs);
    assert!(outputs.iter().take(outputs.len() - 1).all(|&x| x == 0));
    let p1 = outputs.last().unwrap();
    println!("Part One: {p1}");

    let p2 = 0;
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
    }
}
