pub fn q1() {
    println!("fake template for q1");
}

fn number_extractor(input: &str) -> String {
    unimplemented!()
}

fn seprate_first_and_last(input: &str) -> i32 {
    unimplemented!()
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
        assert_eq!(q1::number_extractor("a0000a0"), "0000");
    }

    #[test]
    fn only_extract_first_and_last_digit() {
        assert_eq!(q1::seprate_first_and_last("123"), 13);
        assert_eq!(q1::seprate_first_and_last("120000300"), 10);
        assert_eq!(q1::seprate_first_and_last("0000"), 0);
    }
}
