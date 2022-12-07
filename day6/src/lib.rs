pub fn find_packet_marker(input: &str) -> usize {
    let mut current: usize = 4;
    while !check_different(&input[current-4..current]) {
        current = current + 1;
    }
    return current;
}

pub fn find_message_marker(input: &str) -> usize {
    let inbytes = input.as_bytes();
    
    let mut wstart: usize = 0;
    let mut wend: usize = 0;
    let mut counts: [usize; 27] = [0; 27];

    counts[char_to_ix(inbytes[wstart])] = 1;

    while wend - wstart < 13 {
        wend = wend + 1;
        while counts[char_to_ix(inbytes[wend])] > 0 {
            counts[char_to_ix(inbytes[wstart])] = counts[char_to_ix(inbytes[wstart])] - 1;
            wstart = wstart + 1;
        }
        counts[char_to_ix(inbytes[wend])] = counts[char_to_ix(inbytes[wend])] + 1;
    }

    return wend + 1;
}

fn char_to_ix(byte: u8) -> usize {
    usize::from(byte - 96)
}

fn check_different(slice: &str) -> bool {
    let char0 = slice.chars().nth(0).unwrap();
    let char1 = slice.chars().nth(1).unwrap();
    let char2 = slice.chars().nth(2).unwrap();
    let char3 = slice.chars().nth(3).unwrap();
    return char0 != char1 && char0 != char2 && char0 != char3
        && char1 != char2 && char1 != char3
        && char2 != char3;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_case1() {
        let result = find_packet_marker("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, 5);
    }
    
    #[test]
    fn packet_case2() {
        let result = find_packet_marker("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(result, 6);
    }

    #[test]
    fn packet_case3() {
        let result = find_packet_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(result, 10);
    }

    #[test]
    fn packet_case4() {
        let result = find_packet_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, 11);
    }

    #[test]
    fn message_case1() {
        let result = find_message_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(result, 19);
    }

    #[test]
    fn message_case2() {
        let result = find_message_marker("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, 23);
    }

    #[test]
    fn message_case3() {
        let result = find_message_marker("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(result, 23);
    }

    #[test]
    fn message_case4() {
        let result = find_message_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(result, 29);
    }

    #[test]
    fn message_case5() {
        let result = find_message_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, 26);
    }
}
