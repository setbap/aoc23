use crate::read_file_string;

#[derive(Debug, Default)]
struct Basket {
    red: usize,
    green: usize,
    blue: usize,
}

impl Basket {
    fn create_new_complex_string(input: &str) -> Self {
        let mut basket = Basket::default();
        input.split(";").for_each(|turn| {
            turn.split(",").for_each(|item| {
                let mut number_color = item.trim().split(" ");
                let number = number_color.next().unwrap().parse::<usize>().unwrap();
                let color = number_color.next().unwrap();
                basket.set_fewest_number_of_color(number, color);
            });
        });

        basket
    }

    fn set_fewest_number_of_color(&mut self, count: usize, color: &str) {
        match color {
            "red" => {
                if self.red < count {
                    self.red = count;
                }
            }
            "green" => {
                if self.green < count {
                    self.green = count;
                }
            }
            "blue" => {
                if self.blue < count {
                    self.blue = count;
                }
            }
            _ => panic!("wrong color!?"),
        }
    }

    fn calculate_power(self) -> usize {
        self.red * self.green * self.blue
    }
}
pub fn q2(run_with_test_data: bool) -> String {
    let data: usize = read_file_string(run_with_test_data, 2, 1)
        .lines()
        .map(|line| {
            let mut gameid_record_pair = line.split(":");

            gameid_record_pair.next();
            let record = gameid_record_pair.next().unwrap();

            let basket = Basket::create_new_complex_string(record);
            basket.calculate_power()
        })
        .sum();

    data.to_string()
}
