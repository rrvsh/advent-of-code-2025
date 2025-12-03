pub fn part1(input: &str) -> i64 {
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

fn has_repeat(input: i64) -> bool {
    let input = input.to_string();
    let len = input.len();
    len.is_multiple_of(2) && input[..(len / 2)] == input[(len / 2)..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_repeat_success() {
        let input = 11;
        let actual = has_repeat(input);
        assert!(actual);
    }

    #[test]
    fn has_repeat_failure() {
        let input = 123;
        let actual = has_repeat(input);
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
