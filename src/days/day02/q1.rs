use std::ops::Not;

use crate::read_file_string;

const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

struct Basket;

impl Basket {
    fn is_valid_input(count: usize, color: &str) -> bool {
        match color {
            "red" => count <= RED,
            "green" => count <= GREEN,
            "blue" => count <= BLUE,
            _ => panic!("wrong color!?"),
        }
    }

    fn check_all_inputs(input: &str) -> bool {
        let mut is_valid = true;
        input.split(";").for_each(|turn| {
            turn.split(",").for_each(|item| {
                let mut number_color = item.trim().split(" ");
                let number = number_color.next().unwrap().parse::<usize>().unwrap();
                let color = number_color.next().unwrap();
                if Basket::is_valid_input(number, color).not() {
                    is_valid = false;
                }
            });
        });
        return is_valid;
    }
}

pub fn q1(run_with_test_data: bool) -> String {
    let data: usize = read_file_string(run_with_test_data, 2, 1)
        .lines()
        .map(|line| {
            let mut gameid_record_pair = line.split(":");

            let game_id = gameid_record_pair.next().unwrap();
            let record = gameid_record_pair.next().unwrap();

            let id = game_id.split(" ").last().unwrap().parse::<usize>().unwrap();

            if !Basket::check_all_inputs(record) {
                return 0;
            }

            id
        })
        .sum();

    data.to_string()
}
