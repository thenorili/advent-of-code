use aoc_2022_utils::filepath_read_to_string;

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
    let input_path = env!("CARGO_MANIFEST_DIR").to_owned() + "/input.txt";
    let input = filepath_read_to_string(input_path);
    let output = find_max_run(input);
    print!("{}", output);
}

// TODO criterion to measure benchmarks
// compare to real result? probably not
#[test]
fn main_test() {
    main();
}
