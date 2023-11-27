use crate::util::input_read_to_string;

/* this might be faster to find just the first answer
fn find_max_group(input: String) -> usize
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
*/

/*
 * Expects a string of values separated by newlines arranged in groups separated by blank lines.
 * Returns those groups summed into a vector of values.
 */
fn sum_groups(input: String) -> Vec<usize>
{
    let mut groups = Vec::new();
    let lines = input.lines();
    let mut current = 0;
    for line in lines {
        if line == "" {
            groups.push(current);
            current = 0;
        } else {
            let value: usize = line.parse().unwrap();
            current += value;
        }
    }
    groups
}

pub fn main()
{
    let input = input_read_to_string("1");
    let mut elves: Vec<usize> = sum_groups(input);
    elves.sort();
    let (_small, big) = elves.split_at(elves.len() - 3);
    print!("Part 1 - elf with most calories:\n {:?}\n", big.last().unwrap());
    let sum_of_biggest: usize = big.iter().sum();
    print!("Part 2 - Three elves with most calories:\n {:?}\n", sum_of_biggest);
}

// TODO criterion to measure benchmarks
// compare to real result? probably not
#[test]
fn main_test() {
    main();
}
