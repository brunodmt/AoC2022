use std::fs;

use day5::find_tops;
use day5::find_tops_2;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    find_tops(contents.as_str());
    find_tops_2(contents.as_str());
}

