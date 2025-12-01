
use advent::*;

#[derive(Debug, PartialEq)]
enum DialDirection {
    Left,
    Right,
}

struct InputModel {
    lines: Vec<(DialDirection, i64)>,
}

impl InputModel {
    fn find_password_v1(&self) -> i64 {
        let mut curr_pos = 50;
        let mut curr_count = 0;

        for (dir, dist) in &self.lines {
            let mut this_pos = curr_pos;
            match dir {
                DialDirection::Left => {
                    this_pos -= dist;
                }
                DialDirection::Right => {
                    this_pos += dist;
                }
            }

            while this_pos < 0 { this_pos += 100; }
            while this_pos > 99 { this_pos -= 100; }

            if this_pos == 0 {
                curr_count += 1;
            }

            curr_pos = this_pos;
        }

        curr_count
    }
}

impl From<&str> for InputModel {

    fn from(s: &str) -> Self {
        fn to_dial_direction(s: &str) -> DialDirection {
            if s.starts_with('L') {
                DialDirection::Left
            } else {
                DialDirection::Right
            }
        }

        fn to_dial_distance(s: &str) -> i64 {
            s[1..].trim().parse::<i64>().unwrap()
        }

        let lines = input_as_lines(s).iter()
            .map(|a| (to_dial_direction(a), to_dial_distance(a)))
            .collect::<Vec<_>>();

        InputModel { lines }
    }
}

fn default_input() -> &'static str {
    include_input!(01)
}

pub fn part1() -> String {
    let model = InputModel::from(default_input());

    model.find_password_v1().to_string()
}

pub fn part2() -> String {
    let _model = InputModel::from(default_input());

    "".to_string()
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
        let input = "L10\r\nR5\r\nL3\r\nR2";
        let model = InputModel::from(input);

        assert_eq!(model.lines.len(), 4);
        assert_eq!(model.lines[0].0, DialDirection::Left);
        assert_eq!(model.lines[0].1, 10);
        assert_eq!(model.lines[1].0, DialDirection::Right);
        assert_eq!(model.lines[1].1, 5);
        assert_eq!(model.lines[2].0, DialDirection::Left);
        assert_eq!(model.lines[2].1, 3);
        assert_eq!(model.lines[3].0, DialDirection::Right);
        assert_eq!(model.lines[3].1, 2);
    }

    #[test]
    fn test_find_password_v1() {
        let input = "L68\r\nL30\r\nR48\r\nL5\r\nR60\r\nL55\r\nL1\r\nL99\r\nR14\r\nL82";
        let model = InputModel::from(input);

        assert_eq!(model.find_password_v1(), 3);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "1150");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "");
    }
}
