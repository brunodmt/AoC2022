use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut result: u32 = 0;

    for line in contents.lines() {
        let bytes = line.as_bytes();
        let length = line.len();
        let half = length / 2;
        let mut first = bytes[0..half].to_owned();
        let mut second = bytes[half..length].to_owned();
        first.sort_unstable();
        second.sort_unstable();

        let mut first_it = first.iter();
        let mut second_it = second.iter();

        let mut first_c = first_it.next().unwrap();
        let mut second_c = second_it.next().unwrap();

        while first_c != second_c {
            if first_c < second_c {
                first_c = first_it.next().unwrap();
            } else {
                second_c = second_it.next().unwrap();
            }
        }

        result = result + char_to_priority(*first_c);
    }

    println!("first problem: {}", result);

    let mut result2 = 0;
    let mut lines_it = contents.lines();
    loop {
        if let Some(first_elf) = lines_it.next() {
            let mut first_elf = first_elf.as_bytes().to_owned();
            let mut second_elf = lines_it.next().unwrap().as_bytes().to_owned();
            let mut third_elf = lines_it.next().unwrap().as_bytes().to_owned();

            first_elf.sort_unstable();
            second_elf.sort_unstable();
            third_elf.sort_unstable();

            let mut first_it = first_elf.iter();
            let mut second_it = second_elf.iter();
            let mut third_it = third_elf.iter();

            let mut first_c = first_it.next().unwrap();
            let mut second_c = second_it.next().unwrap();
            let mut third_c = third_it.next().unwrap();

            while first_c != second_c || second_c != third_c || third_c != first_c {
                if first_c < second_c {
                    if first_c < third_c {
                        first_c = first_it.next().unwrap();
                    } else if first_c > third_c {
                        third_c = third_it.next().unwrap();
                    } else {
                        first_c = first_it.next().unwrap();
                        third_c = third_it.next().unwrap();
                    }
                } else if first_c > second_c {
                    if second_c < third_c {
                        second_c = second_it.next().unwrap();
                    } else if second_c > third_c {
                        third_c = third_it.next().unwrap();
                    } else {
                        second_c = second_it.next().unwrap();
                        third_c = third_it.next().unwrap();
                    }
                } else {
                    if first_c < third_c {
                        first_c = first_it.next().unwrap();
                        second_c = second_it.next().unwrap();
                    } else if first_c > third_c {
                        third_c = third_it.next().unwrap();
                    }
                }
            }

            result2 = result2 + char_to_priority(*first_c);
        } else {
            break;
        }
    }

    println!("result2: {}", result2);
}

fn char_to_priority(char: u8) -> u32 {
    let char = u32::from(char);
    if char >= 65 && char <= 90 {
        return char - 65 + 27;
    } else {
        return char - 97 + 1;
    }
}
