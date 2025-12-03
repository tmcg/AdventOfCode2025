
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

    fn max_joltage(&self, part2: bool) -> i64 {
        match part2 {
            false => self.max_joltage_part1(),
            true => self.max_joltage_part2(),
        }
    }

    fn max_joltage_part1(&self) -> i64 {
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

    fn max_joltage_part2(&self) -> i64 {
        self.find_highest(12)
    }

    fn find_highest(&self, digits: usize) -> i64 {
        let mut v: Vec<i64> = Vec::new();
        self.collect_highest(&mut v, 0, digits);

        //println!("find_highest: blen = {}, v = {:?}", self.batteries.len(), v);

        v.iter().map(|n| n.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap()
    }

    fn collect_highest(&self, v: &mut Vec<i64>, ix_start: usize, digits: usize) {
        let blen = self.batteries.len();
        let start = ix_start as i64;
        let limit = (blen - digits) as i64;

        //println!("collect_highest: start = {}, limit = {}, digits = {}, vlen={}, v = {:?}", start, limit, digits, v.len(), v);

        if digits == 0 || ix_start >= self.batteries.len() {
            return;
        }

        let max_highest = self.batteries.iter().filter(|b| b.index >= start && b.index <= limit).max_by_key(|b| b.capacity).unwrap();
        let first_highest = self.batteries.iter().find(|b| b.index >= start && b.capacity == max_highest.capacity).unwrap();

        v.push(first_highest.capacity);

        let fh = first_highest.index as usize;
        self.collect_highest(v, fh + 1, digits - 1);
    }
}


fn default_input() -> &'static str {
    include_input!(03)
}

pub fn part1() -> String {
    let model = InputModel::from(default_input());

    model.banks.iter().map(|b| b.max_joltage(false)).sum::<i64>().to_string()
}

pub fn part2() -> String {
    let model = InputModel::from(default_input());
    model.banks.iter().map(|b| b.max_joltage(true)).sum::<i64>().to_string()
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
    fn test_max_joltage_part1() {
        let input = "987654321111111\r\n811111111111119\r\n234234234234278\r\n818181911112111";
        let model = InputModel::from(input);

        assert_eq!(model.banks.len(), 4);
        assert_eq!(model.banks[0].max_joltage(false), 98);
        assert_eq!(model.banks[1].max_joltage(false), 89);
        assert_eq!(model.banks[2].max_joltage(false), 78);
        assert_eq!(model.banks[3].max_joltage(false), 92);
    }

    #[test]
    fn test_max_joltage_part2() {
        let input = "987654321111111\r\n811111111111119\r\n234234234234278\r\n818181911112111";
        let model = InputModel::from(input);

        assert_eq!(model.banks.len(), 4);
        assert_eq!(model.banks[0].max_joltage(true), 987654321111);
        assert_eq!(model.banks[1].max_joltage(true), 811111111119);
        assert_eq!(model.banks[2].max_joltage(true), 434234234278);
        assert_eq!(model.banks[3].max_joltage(true), 888911112111);
    }

    #[test]
    fn test_find_highest() {

        assert_eq!(BatteryBank::from("123456789").find_highest(3), 789);
        assert_eq!(BatteryBank::from("8119").find_highest(2), 89);
        assert_eq!(BatteryBank::from("987654321111111").find_highest(2), 98);
        assert_eq!(BatteryBank::from("811111111111119").find_highest(2), 89);
        assert_eq!(BatteryBank::from("234234234234278").find_highest(2), 78);
        assert_eq!(BatteryBank::from("818181911112111").find_highest(2), 92);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "17100");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "170418192256861");
    }
}
