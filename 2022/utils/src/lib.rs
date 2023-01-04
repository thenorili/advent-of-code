use std::fs;
use std::path;

/*
 * For the advent of code, we only ever need an input.txt as input.
 * This macro grabs CARGO_MANIFEST_DIR/input.txt and returns a string.
 * use like...
 *
 * let input_path = env!("CARGO_MANIFEST_DIR").to_owned() + "/input.txt";
 * let input_txt = filepath_read_to_string(input_path);
 */
pub fn filepath_read_to_string(input: String) -> String
{
    let path = path::Path::new(&input);
    assert!(path.exists(), "input.txt was not found in the CARGO_MANIFEST_DIR");

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    return contents;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input_path = env!("CARGO_MANIFEST_DIR").to_owned() + "/input.txt";
        let input_txt = filepath_read_to_string(input_path);
        assert_eq!(input_txt, "sample input\n");
    }
}
