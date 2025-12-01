
use advent::*;

#[derive(Debug, PartialEq)]
enum DialDirection {
    Left,
    Right,
}

struct Dial {
    position: i64,
    zero_hits: i64,
}

impl Dial {
    fn new(pos: i64) -> Self {
        Dial { position: pos, zero_hits: 0 }
    }

    fn click_v1(&mut self, direction: &DialDirection, distance: i64) {
        let dist_factor = Dial::dist_factor(direction);
        let new_pos = self.position + (distance * dist_factor);

        if new_pos % 100 == 0 {
            self.zero_hits += 1;
        }

        self.position = new_pos.rem_euclid(100);
    }

    fn click_v2(&mut self, direction: &DialDirection, distance: i64) {
        let dist_factor = Dial::dist_factor(direction);
        let new_pos = self.position + (distance * dist_factor);

        let mut old_pos = self.position;
        while old_pos != new_pos {
            old_pos += dist_factor;
            if old_pos % 100 == 0 {
                self.zero_hits += 1;
            }
        }

        self.position = new_pos.rem_euclid(100);
    }

    fn dist_factor(direction: &DialDirection) -> i64 {
        match direction {
            DialDirection::Left => -1,
            DialDirection::Right => 1,
        }
    }
}

struct InputModel {
    lines: Vec<(DialDirection, i64)>,
}

impl InputModel {
    fn find_password_v1(&self) -> i64 {
        let mut dial = Dial::new(50);

        for (dir, dist) in &self.lines {
            dial.click_v1(dir, *dist);
        }

        dial.zero_hits
    }

    fn find_password_v2(&self) -> i64 {
        let mut dial = Dial::new(50);

        for (dir, dist) in &self.lines {
            dial.click_v2(dir, *dist);
        }

        dial.zero_hits
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
    let model = InputModel::from(default_input());

    model.find_password_v2().to_string()
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
    fn test_find_password_v2() {
        let input = "L68\r\nL30\r\nR48\r\nL5\r\nR60\r\nL55\r\nL1\r\nL99\r\nR14\r\nL82";
        let model = InputModel::from(input);

        assert_eq!(model.find_password_v2(), 6);
    }

    #[test]
    fn test_dial_click_v2_slow() {
        let mut dial = Dial::new(3);

        dial.click_v2(&DialDirection::Left, 1);
        assert_eq!(dial.position, 2);
        assert_eq!(dial.zero_hits, 0);

        dial.click_v2(&DialDirection::Left, 1);
        assert_eq!(dial.position, 1);
        assert_eq!(dial.zero_hits, 0);

        dial.click_v2(&DialDirection::Left, 1);
        assert_eq!(dial.position, 0);
        assert_eq!(dial.zero_hits, 1);

        dial.click_v2(&DialDirection::Left, 1);
        assert_eq!(dial.position, 99);
        assert_eq!(dial.zero_hits, 1);
    }

    #[test]
    fn test_dial_click_v2_sample() {
        let mut dial = Dial::new(50);

        dial.click_v2(&DialDirection::Left, 68);
        assert_eq!(dial.position, 82);
        assert_eq!(dial.zero_hits, 1);

        dial.click_v2(&DialDirection::Left, 30);
        assert_eq!(dial.position, 52);
        assert_eq!(dial.zero_hits, 1);

        dial.click_v2(&DialDirection::Right, 48);
        assert_eq!(dial.position, 0);
        assert_eq!(dial.zero_hits, 2);

        dial.click_v2(&DialDirection::Left, 5);
        assert_eq!(dial.position, 95);
        assert_eq!(dial.zero_hits, 2);

        dial.click_v2(&DialDirection::Right, 60);
        assert_eq!(dial.position, 55);
        assert_eq!(dial.zero_hits, 3);

        dial.click_v2(&DialDirection::Left, 55);
        assert_eq!(dial.position, 0);
        assert_eq!(dial.zero_hits, 4);

        dial.click_v2(&DialDirection::Left, 1);
        assert_eq!(dial.position, 99);
        assert_eq!(dial.zero_hits, 4);

        dial.click_v2(&DialDirection::Left, 99);
        assert_eq!(dial.position, 0);
        assert_eq!(dial.zero_hits, 5);

        dial.click_v2(&DialDirection::Right, 14);
        assert_eq!(dial.position, 14);
        assert_eq!(dial.zero_hits, 5);

        dial.click_v2(&DialDirection::Left, 82);
        assert_eq!(dial.position, 32);
        assert_eq!(dial.zero_hits, 6);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "1150");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "6738");
    }
}
