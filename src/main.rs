use std::io::Write;

fn next_ducci(ducci: &[i8; 4]) -> [i8; 4] {
    let mut result = [0; 4];
    result[0] = (ducci[0] - ducci[1]).abs();
    result[1] = (ducci[1] - ducci[2]).abs();
    result[2] = (ducci[2] - ducci[3]).abs();
    result[3] = (ducci[3] - ducci[0]).abs();
    return result;
}

fn generate_swaps(ducci: &[i8; 4]) -> [[i8; 4]; 2] {
    return [[ducci[1], ducci[0], ducci[2], ducci[3]],
            [ducci[0], ducci[2], ducci[1], ducci[3]]];
}

fn is_zero_convergence(ducci: &[i8; 4]) -> bool {
    // Check if the Ducci sequence has converged to (0, 0, 0, 0)
    return ducci[0] == 0 && ducci[1] == 0 && ducci[2] == 0 && ducci[3] == 0;
}

fn is_checker_convergence(ducci: &[i8; 4]) -> bool {
    // Check if the Ducci sequence has converged to (0, a, 0, a)
    return ducci[0] == 0 && ducci[2] == 0 && ducci[1] == ducci[3];
}

fn is_ladder_convergence(ducci: &[i8; 4]) -> bool {
    // Check if the Ducci sequence has converged to (0, a, 2a, a)
    return ducci[0] == 0 && ducci[2] == 2 * ducci[1] && ducci[3] == ducci[1];
}

fn generate_rotations(ducci: &[i8; 4]) -> [[i8; 4]; 4] {
    // 90 degree rotation
    let ninety = [ducci[3], ducci[0], ducci[1], ducci[2]];
    // 180 degree rotation
    let one_eighty = [ducci[2], ducci[3], ducci[0], ducci[1]];
    // 270 degree rotation
    let two_seventy = [ducci[1], ducci[2], ducci[3], ducci[0]];

    return [*ducci, ninety, one_eighty, two_seventy]
}

fn is_terminal(ducci: &[i8; 4]) -> bool {
    if is_zero_convergence(ducci) {
        return true;
    }

    for rotation in generate_rotations(ducci) {
        if is_checker_convergence(&rotation) {
            return true;
        }
        if is_ladder_convergence(&rotation) {
            return true;
        }
    }

    return false;
}

fn calculate_paths(ducci: &[i8; 4]) -> Vec<([i8; 4], u8)> {
    let next_state = next_ducci(ducci);

    if is_terminal(&next_state) {
        return vec![(next_state, 0)];
    }

    let swaps = generate_swaps(&next_state);

    let swap_results_one = calculate_paths(&swaps[0]);
    let swap_results_two = calculate_paths(&swaps[1]);

    let total_length = swap_results_one.len() + swap_results_two.len();
    let mut terminals = Vec::with_capacity(total_length);

    for swap_result in swap_results_one.iter() {
        terminals.push((swap_result.0, swap_result.1 + 1));
    }

    for swap_result in swap_results_two.iter() {
        terminals.push((swap_result.0, swap_result.1 + 1));
    }

    return terminals;
}

fn write_tuple(tuple: [i8; 4]) -> String {
    let mut result = String::new();

    result.push_str("(");

    for i in tuple.iter() {
        result.push_str(&format!("{},", i));
    }

    result.pop();

    result.push_str(")");

    return result;
}

fn write_paths(paths: Vec<([i8; 4], u8)>) -> String {
    let mut result = String::new();

    result.push_str("[");

    for path in paths.iter() {
        let tuple_string = write_tuple(path.0);
        result.push_str(&format!("{}:{},", tuple_string,  path.1));
    }

    result.pop();

    result.push_str("]");

    return result;
}

fn main() {
    let start_time = std::time::Instant::now();

    const MAX_INT: i8 = 40;

    // Open a file for writing
    let mut file = std::fs::File::create("results/ducci_paths.txt").unwrap();

    for a in 0..MAX_INT {
        for b in 0..MAX_INT {
            for c in 0..MAX_INT {
                for d in 0..MAX_INT {
                    let starting_ducci = [a, b, c, d];
                    let paths = calculate_paths(&starting_ducci);
                    let path_string = write_paths(paths);
                    let tuple_string = write_tuple(starting_ducci);
                    let output_string = format!{"{}:{}\n", tuple_string, path_string};
                    file.write_all(output_string.as_bytes()).unwrap();
                }
            }
        }
    }

    let end_time = std::time::Instant::now();
    let elapsed = end_time.duration_since(start_time);
    println!("Elapsed: {:?}", elapsed);
}
