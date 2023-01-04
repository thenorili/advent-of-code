use std::fs;
use std::env::args;
use std::path;

// TODO turn input.txt import into library for other AoC puzzles
fn args_to_vec() -> Vec<String>
{
    let args: Vec<String> = args().collect();
    return args
}

fn filepath_read_to_string(input: String) -> String
{
    let path = path::Path::new(&input);
    debug_assert!(path.exists());

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    return contents;
}

fn find_max_run(input: String) -> usize
{
    // let mut elves = vec![];
    let mut current: usize = 0;
    let mut max: usize = 0;
    let lines = input.lines();
    for line in lines {
        if line == "" {
            // elves.push(current);
            if current > max {
                max = current;
            }
            current = 0;

        } else {
            let cal: usize = line.parse().unwrap();
            current += cal;
        }
    }
    return max
}

fn main()
{
    let args = args_to_vec();
    let input = filepath_read_to_string(args[1].to_owned());
    find_max_run(input);
}

// TODO criterion to measure benchmarks
#[test]
fn main_test() {
    let proj_path = env!("CARGO_MANIFEST_DIR").to_owned();
    let input_path = proj_path + "/input.txt";
    let input = filepath_read_to_string(input_path);
    find_max_run(input);
}
