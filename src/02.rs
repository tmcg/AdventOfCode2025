
use advent::*;

struct ProductRange {
    start: i64,
    end: i64,
}

struct InputModel {
    ranges: Vec<ProductRange>,
}

impl From<&str> for InputModel {
    fn from(s: &str) -> Self {
        let lines = input_as_lines(s);
        let parts: Vec<&str> = lines[0].split(',').collect();
        let ranges: Vec<ProductRange> = parts.into_iter().map(|x| ProductRange::from(x.trim())).collect();
        InputModel { ranges }
    }
}

impl From<&str> for ProductRange {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('-').collect();
        let start = parts[0].trim().parse::<i64>().expect("Unable to parse start of range");
        let end = parts[1].trim().parse::<i64>().expect("Unable to parse end of range");
        ProductRange { start, end }
    }
}

impl ProductRange {
    fn is_valid_id(id: i64) -> bool {
        let s = id.to_string();

        if s.len().is_multiple_of(2) {
            let half_len = s.len() / 2;
            let (first_half, second_half) = s.split_at(half_len);
            return first_half != second_half;
        }

        true
    }
}

impl InputModel {

    fn sum_invalid_ids(&self) -> i64 {
        let mut sum = 0;

        for range in &self.ranges {
            //println!("Checking range {}-{}", range.start, range.end);
            for id in range.start..=range.end {
                if !ProductRange::is_valid_id(id) {
                    sum += id;
                }
            }
        }

        sum
    }
}

fn default_input() -> &'static str {
    include_input!(02)
}

pub fn part1() -> String {
    let model = InputModel::from(default_input());

    model.sum_invalid_ids().to_string()
}

pub fn part2() -> String {
    //let model = InputModel::from(default_input());
    //model.lines.len().to_string()
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
    fn test_product_range_from() {
        let input = "10-20";
        let range = ProductRange::from(input);

        assert_eq!(range.start, 10);
        assert_eq!(range.end, 20);
    }

    #[test]
    fn test_input_model_from() {
        let input = "1-3,5-7,10-15";
        let model = InputModel::from(input);

        assert_eq!(model.ranges.len(), 3);
        assert_eq!(model.ranges[0].start, 1);
        assert_eq!(model.ranges[0].end, 3);
        assert_eq!(model.ranges[1].start, 5);
        assert_eq!(model.ranges[1].end, 7);
        assert_eq!(model.ranges[2].start, 10);
        assert_eq!(model.ranges[2].end, 15);
    }

    #[test]
    fn test_is_valid_id() {
        assert_eq!(ProductRange::is_valid_id(1212), false);
        assert_eq!(ProductRange::is_valid_id(123123), false);
        assert_eq!(ProductRange::is_valid_id(123456), true);
        assert_eq!(ProductRange::is_valid_id(112233), true);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "53420042388");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "zz");
    }
}
