use std::fs;
use std::path;

/*
 * For the advent of code, we only ever need an input.txt as input.
 * This macro accepts a relative path from the cargo manifest directory
 * and returns a string.
 */
fn filepath_read_to_string(dir: &str, filename: &str) -> String
{
    let root = path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = root.join(dir.to_owned()).join(filename.to_owned() + ".txt");
    assert!(path.exists(), "File not found: {}", path.display());
    let contents = fs::read_to_string(path)
        .expect("Unable to read the input file.");
    return contents;
}

pub fn input_read_to_string(filename: &str) -> String
{
    return filepath_read_to_string("input", filename);
}

pub fn problem_print(filename: &str)
{
    println!("{}", filepath_read_to_string("problem", filename));
}


mod tests {
    #[test]
    fn it_works() {
        let input_txt = filepath_read_to_string("util".into());
        assert_eq!(input_txt, "sample input\n");
    }
}
