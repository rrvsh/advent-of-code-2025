pub fn part1(input: &str) -> i64 {
    parse_input(input)
        .into_iter()
        .filter(|x| has_one_repeat(*x))
        .sum::<i64>()
}

pub fn part2(input: &str) -> i64 {
    parse_input(input)
        .into_iter()
        .filter(|x| has_repeat(*x))
        .sum::<i64>()
}

fn parse_input(input: &str) -> Vec<i64> {
    let mut acc: Vec<i64> = Vec::new();
    for part in input.split(',').map(str::trim) {
        let tmp: Vec<i64> = part
            .split('-')
            .map(|x| x.parse::<i64>().expect("parse int error"))
            .collect();
        for i in tmp[0]..=tmp[1] {
            acc.push(i);
        }
    }
    acc
}

fn has_one_repeat(input: i64) -> bool {
    let input = input.to_string();
    let len = input.len();
    len.is_multiple_of(2) && input[..(len / 2)] == input[(len / 2)..]
}

fn split_by_sublength(input: &str, sub_len: usize) -> Vec<&str> {
    if input.len() == sub_len {
        vec![input]
    } else {
        let mut acc = vec![];
        let (good_chunk, bad_chunk) = input.split_at(sub_len);
        acc.push(good_chunk);
        acc.append(&mut split_by_sublength(bad_chunk, sub_len));
        acc
    }
}

fn has_repeat(input: i64) -> bool {
    if has_one_repeat(input) {
        return true;
    }
    let input = input.to_string();
    let len = input.len();
    for sub_len in 1..len {
        if len.is_multiple_of(sub_len) {
            let chunks: Vec<i64> = split_by_sublength(&input, sub_len)
                .into_iter()
                .map(|x| x.parse::<i64>().expect("parse int error"))
                .collect();
            let first = chunks[0];
            if chunks.into_iter().all(|x| x == first) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_one_repeat_success() {
        let input = 11;
        let actual = has_one_repeat(input);
        assert!(actual);
    }

    #[test]
    fn has_one_repeat_failure() {
        let input = 123;
        let actual = has_one_repeat(input);
        assert!(!actual);
    }

    #[test]
    fn parse_input_success() {
        let input = "11-22,95-115";
        let actual = parse_input(input);
        let mut expected = Vec::new();
        for i in 11..=22 {
            expected.push(i);
        }
        for i in 95..=115 {
            expected.push(i);
        }
        assert_eq!(actual, expected);
    }

    #[test]
    fn part1_success() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        let actual = part1(input);
        let expected = 1_227_775_554;
        assert_eq!(actual, expected);
    }
}
