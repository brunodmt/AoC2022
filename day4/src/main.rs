use std::fs;
use day4::find_containing_pairs;
use day4::find_overlaps;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    find_containing_pairs(contents.as_str());
    find_overlaps(contents.as_str());
}
