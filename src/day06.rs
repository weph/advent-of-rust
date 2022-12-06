use std::collections::HashSet;
use std::fs;

fn position_after_unique_characters(input: &str, len: usize) -> Result<usize, String> {
    if len > input.len() {
        return Err(format!("String '{input}' is shorter than {len} characters"));
    }

    for index in 0..input.len() - len {
        let chars: HashSet<char> = HashSet::from_iter((&input[index..index + len]).chars());

        if chars.len() == len as usize {
            return Ok(index + len);
        }
    }

    return Err(format!("String '{input}' does not contain a series of {len} unique characters"));
}

fn main() {
    let input = fs::read_to_string("input/day06.txt").unwrap();

    println!("Part One: {}", position_after_unique_characters(&input, 4).unwrap());
    println!("Part Two: {}", position_after_unique_characters(&input, 14).unwrap());
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn no_unique_characters() {
        assert_eq!(
            "String 'aaaa' does not contain a series of 2 unique characters",
            position_after_unique_characters("aaaa", 2).unwrap_err()
        );
    }

    #[test]
    fn input_is_shorter_than_length() {
        assert_eq!(
            "String 'ab' is shorter than 3 characters",
            position_after_unique_characters("ab", 3).unwrap_err()
        );
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4, 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 4, 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11)]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14, 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 14, 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 14, 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14, 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14, 26)]
    fn position_test(#[case] input: &str, #[case] len: usize, #[case] expected: usize) {
        assert_eq!(expected, position_after_unique_characters(input, len).unwrap());
    }
}
