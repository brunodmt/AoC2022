use regex::Regex;

pub fn find_containing_pairs(input: &str) {
    let re = Regex::new(r"(?m)^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut cont_pairs = 0;

    for cap in re.captures_iter(input) {
        let start1 = &cap[1].parse::<usize>().unwrap();
        let end1 = &cap[2].parse::<usize>().unwrap();
        let start2 = &cap[3].parse::<usize>().unwrap();
        let end2 = &cap[4].parse::<usize>().unwrap();

        if (start1 <= start2 && end1 >= end2) || (start1 >= start2 && end1 <= end2) {
            cont_pairs = cont_pairs + 1;
        }
    }

    println!("result: {}", cont_pairs);
}

pub fn find_overlaps(input: &str) {
    let re = Regex::new(r"(?m)^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut cont_pairs = 0;

    for cap in re.captures_iter(input) {
        let start1 = &cap[1].parse::<usize>().unwrap();
        let end1 = &cap[2].parse::<usize>().unwrap();
        let start2 = &cap[3].parse::<usize>().unwrap();
        let end2 = &cap[4].parse::<usize>().unwrap();

        if (start1 >= start2 && start1 <= end2) || (end1 >= start2 && end1 <= end2)
                || (start1 <= start2 && end1 >= end2) || (start1 >= start2 && end1 <= end2) {
            cont_pairs = cont_pairs + 1;
        }
    }

    println!("result: {}", cont_pairs);
}