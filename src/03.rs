
use advent::*;
use std::fmt;

#[derive(Clone, Copy)]
struct Battery {
    capacity: i64,
    index: i64,
}

#[derive(Clone)]
struct BatteryBank {
    batteries: Vec<Battery>,
}

struct InputModel {
    banks: Vec<BatteryBank>,
}

impl From<&str> for Battery {
    fn from(s: &str) -> Self {
        let parts = s.split(',').collect::<Vec<&str>>();
        let capacity = parts[0].trim().parse::<i64>().expect("Unable to parse battery capacity");
        let index = parts[1].trim().parse::<i64>().expect("Unable to parse battery index");
        Battery { capacity, index }
    }
}

impl From<&str> for BatteryBank {
    fn from(s: &str) -> Self {
        let bat_map = |(i, c): (usize, char)| Battery::from(format!("{},{}", c, i).as_str());
        let batteries: Vec<Battery> = s.chars().enumerate().map(bat_map).collect();
        BatteryBank { batteries }
    }
}

impl fmt::Display for BatteryBank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let caps: Vec<String> = self.batteries.iter().map(|b| b.capacity.to_string()).collect();
        write!(f, "{}", caps.join(""))
    }
}

impl From<&str> for InputModel {
    fn from(s: &str) -> Self {
        let bank_map = |line: String| BatteryBank::from(line.as_str());
        let banks: Vec<BatteryBank> = input_as_lines(s).into_iter().map(bank_map).collect();
        InputModel { banks }
    }
}



impl BatteryBank {
    fn size(&self) -> usize {
        self.batteries.len()
    }

    fn max_joltage(&self) -> i64 {
        let mut result = 0;
        if self.size() >= 2 {
            let highest = self.batteries.iter().max_by_key(|b| b.capacity).unwrap();
            let highest_left = self.batteries.iter().filter(|b| b.index < highest.index).max_by_key(|b| b.capacity);
            let highest_right = self.batteries.iter().filter(|b| b.index > highest.index).max_by_key(|b| b.capacity);

            let left_capacity = match highest_left {
                Some(b) => b.capacity * 10 + highest.capacity,
                None => 0,
            };

            let right_capacity = match highest_right {
                Some(b) => highest.capacity * 10 + b.capacity,
                None => 0,
            };

            result = left_capacity.max(right_capacity);
        }

        result
    }
}

fn default_input() -> &'static str {
    include_input!(03)
}

pub fn part1() -> String {
    let model = InputModel::from(default_input());

    model.banks.iter().map(|b| b.max_joltage()).sum::<i64>().to_string()
}

pub fn part2() -> String {
    //let model = InputModel::from(default_input());
    //model.banks.len().to_string()

    String::from("zz")
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_model_from() {
        let input = "123\r\n45";
        let model = InputModel::from(input);

        assert_eq!(model.banks.len(), 2);
        assert_eq!(model.banks[0].batteries.len(), 3);
        assert_eq!(model.banks[1].batteries.len(), 2);
        assert_eq!(model.banks[0].batteries[0].capacity, 1);
        assert_eq!(model.banks[0].batteries[1].capacity, 2);
        assert_eq!(model.banks[0].batteries[2].capacity, 3);
        assert_eq!(model.banks[1].batteries[0].capacity, 4);
        assert_eq!(model.banks[1].batteries[1].capacity, 5);
    }

    #[test]
    fn test_max_joltage() {
        let input = "987654321111111\r\n811111111111119\r\n234234234234278\r\n818181911112111";
        let model = InputModel::from(input);

        assert_eq!(model.banks.len(), 4);
        assert_eq!(model.banks[0].max_joltage(), 98);
        assert_eq!(model.banks[1].max_joltage(), 89);
        assert_eq!(model.banks[2].max_joltage(), 78);
        assert_eq!(model.banks[3].max_joltage(), 92);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "17100");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "zz");
    }
}
