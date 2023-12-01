use regex::Regex;
use std::fs;

pub fn q1(run_with_test_data: bool) -> String {
    let path = if run_with_test_data {
        "src/days/day01/q1_input_question.txt"
    } else {
        "src/days/day01/q1_input_example.txt"
    };
    let data = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|input| {
            let extracted_number = number_extractor(input);
            seprate_first_and_last(extracted_number.as_str())
        })
        .sum::<i32>();

    data.to_string()
}

fn number_extractor(input: &str) -> String {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input).map(|mat| mat.as_str()).collect()
}

fn seprate_first_and_last(input: &str) -> i32 {
    match input.len() {
        1 => input.parse::<i32>().unwrap() * 11,
        2 => input.parse().unwrap(),
        _ => {
            let right_digit = input.chars().rev().next().unwrap().to_digit(10).unwrap() as i32;
            let left_digit = input.chars().next().unwrap().to_digit(10).unwrap() as i32;
            (left_digit * 10) + right_digit
        }
    }
}

#[cfg(test)]
mod q1_test {
    use super::super::q1;

    #[test]
    fn extract_all_numbers_in_string() {
        assert_eq!(q1::number_extractor("hello123asd"), "123");
        assert_eq!(q1::number_extractor("23a3i4n9"), "23349");
        assert_eq!(q1::number_extractor("993"), "993");
        assert_eq!(q1::number_extractor("aaaaaa9aaaaaaaa9aaaaaaaaa9"), "999");
        assert_eq!(q1::number_extractor("aaaaaa9aaaaaaaa9aaaaaaaaa9"), "999");
        assert_eq!(q1::number_extractor("a0000a0"), "00000");
    }

    #[test]
    fn only_extract_first_and_last_digit() {
        assert_eq!(q1::seprate_first_and_last("123"), 13);
        assert_eq!(q1::seprate_first_and_last("120000300"), 10);
        assert_eq!(q1::seprate_first_and_last("0000"), 0);
    }
}
