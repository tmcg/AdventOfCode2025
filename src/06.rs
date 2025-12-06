
use advent::*;

#[derive(Debug, PartialEq, Eq)]
enum MathOperator {
    Add,
    Multiply,
}
 

#[derive(Debug, PartialEq, Eq)]
struct MathProblem {
    numbers: Vec<i64>,
    operator: MathOperator,
}

struct MathWorksheet {
    problems: Vec<MathProblem>
}

impl From<&str> for MathWorksheet {
    fn from(s: &str) -> Self {
        let lines = input_as_lines(s);

        let line_tokens = lines.iter().map(|line| line.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();

        let mut problems: Vec<MathProblem> = Vec::new();

        if let Some(ops) = line_tokens.last() {
            let problem_count = ops.len();

            for i in 0..problem_count {
                let mut numbers: Vec<i64> = Vec::new();

                for line in line_tokens.iter().take(line_tokens.len() - 1) {
                    let num = line[i].parse::<i64>().unwrap();
                    numbers.push(num);
                }

                problems.push(MathProblem {
                    numbers,
                    operator: match ops[i] {
                        "*" => MathOperator::Multiply,
                        "+" => MathOperator::Add,
                        _ => panic!("Unknown operator {}", ops[i]),
                    },
                });
            }
        }

        MathWorksheet { problems }
    }
}

impl MathProblem {
    fn solve(&self) -> i64 {
        match self.operator {
            MathOperator::Add => self.numbers.iter().sum(),
            MathOperator::Multiply => self.numbers.iter().product(),
        }
    }
}

fn default_input() -> &'static str {
    include_input!(06)
}

pub fn part1() -> String {
    let ws = MathWorksheet::from(default_input());

    ws.problems.iter().map(|p| p.solve()).sum::<i64>().to_string()
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
    fn test_worksheet_from() {
        let input = "123 328  51 64 \r\n 45 64  387 23 \r\n  6 98  215 314\r\n*   +   *   +  ";
        let ws = MathWorksheet::from(input);
        assert_eq!(ws.problems.len(), 4);
        assert_eq!(ws.problems[0], MathProblem { numbers: vec![123, 45, 6], operator: MathOperator::Multiply });
        assert_eq!(ws.problems[1], MathProblem { numbers: vec![328, 64, 98], operator: MathOperator::Add });
        assert_eq!(ws.problems[2], MathProblem { numbers: vec![51, 387, 215], operator: MathOperator::Multiply });
        assert_eq!(ws.problems[3], MathProblem { numbers: vec![64, 23, 314], operator: MathOperator::Add });
    }

    #[test]
    fn test_worksheet_solve() {
        let input = "123 328  51 64 \r\n 45 64  387 23 \r\n  6 98  215 314\r\n*   +   *   +  ";
        let ws = MathWorksheet::from(input);
        assert_eq!(ws.problems[0].solve(), 123 * 45 * 6);
        assert_eq!(ws.problems[1].solve(), 328 + 64 + 98);
        assert_eq!(ws.problems[2].solve(), 51 * 387 * 215);
        assert_eq!(ws.problems[3].solve(), 64 + 23 + 314);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "7644505810277");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "zz");
    }
}
