
use advent::*;

struct InputModel {
    lines: Vec<String>,
}

impl From<&str> for InputModel {
    fn from(s: &str) -> Self {
        let lines = input_as_lines(s);
        InputModel { lines }
    }
}

fn default_input() -> &'static str {
    include_input!(00)
}

pub fn part1() -> String {
    let model = InputModel::from(default_input());
    model.lines.len().to_string()
}

pub fn part2() -> String {
    let model = InputModel::from(default_input());
    model.lines.len().to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "zz");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "zz");
    }
}
