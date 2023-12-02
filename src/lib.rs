use std::fs;

pub mod days;

pub fn read_file_string(run_with_test_data: bool, day_number: i32, day_part: i8) -> String {
    let data_type = if run_with_test_data { "e" } else { "q" };
    let path = format!("data/day{:0>2}/{}{}.txt", day_number, data_type, day_part);
    fs::read_to_string(path).unwrap()
}
