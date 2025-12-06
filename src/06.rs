
use advent::*;

#[derive(Debug, Default, PartialEq, Eq)]
enum MathOperator {
    #[default]
    Add,
    Multiply,
}
 
#[derive(Debug, Default, PartialEq, Eq)]
struct MathProblem {
    numbers: Vec<i64>,
    operator: MathOperator,
}

struct MathWorksheet {
    part1_problems: Vec<MathProblem>,
    part2_problems: Vec<MathProblem>,
}

impl TryFrom<Option<char>> for MathOperator {
    type Error = ();
    fn try_from(value: Option<char>) -> Result<Self, Self::Error> {
        match value {
            Some('+') => Ok(MathOperator::Add),
            Some('*') => Ok(MathOperator::Multiply),
            _ => Err(()),
        }
    }
}

impl From<&str> for MathWorksheet {
    fn from(s: &str) -> Self {
        let input = input_as_lines(s);
        let part1_problems = MathWorksheet::parse_input_part1(&input);
        let part2_problems = MathWorksheet::parse_input_part2(&input);

        MathWorksheet { part1_problems, part2_problems }
    }
}

impl MathWorksheet {
    fn parse_input_part1(lines: &[String]) -> Vec<MathProblem> {

        // Split each line, ignoring whitespace in part 1
        let line_tokens = lines.iter().map(|line| line.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();

        let mut problems: Vec<MathProblem> = Vec::new();

        // The last line contains the operators, and also the count of problems
        if let Some(ops) = line_tokens.last() {
            let problem_count = ops.len();

            // For each problem, collate the numbers and operator
            // Each problem is a set of columns in the input, separated by an empty column
            for i in 0..problem_count {
                if let Ok(operator) = MathOperator::try_from(ops[i].chars().next()) {
                    let mut numbers: Vec<i64> = Vec::new();

                    for line in line_tokens.iter().take(line_tokens.len() - 1) {
                        let num = line[i].parse::<i64>().unwrap();
                        numbers.push(num);
                    }

                    problems.push(MathProblem {
                        numbers,
                        operator,
                    });
                } else {
                    panic!("Unknown operator {}", ops[i]);
                }
            }
        }

        problems
    }

    fn parse_input_part2(lines: &[String]) -> Vec<MathProblem> {

        // Pivot the input lines, so columns become rows
        let pivoted = MathWorksheet::pivot_input(lines);

        // For each problem, collate the numbers and operator
        // Each problem is now a set of rows in the pivoted input, separated by an empty row
        let mut problems: Vec<MathProblem> = Vec::new();
        let mut problem = MathProblem::default();
        for v in &pivoted {
            let mut vt = v.trim();

            if vt.is_empty() {
                problems.push(problem);
                problem = MathProblem::default();
                continue;
            }

            if let Ok(operator) = MathOperator::try_from(vt.chars().last()) {
                problem.operator = operator;
                vt = vt[..vt.len()-1].trim();
            }

            problem.numbers.push(vt.parse::<i64>().unwrap());
        }

        problems.push(problem);

        //println!("Pivoted Lines: {:?}", pivoted);
        //println!("Part2 Problems: {:?}", problems);

        problems
    }

    fn pivot_input(lines: &[String]) -> Vec<String> {
        let width = lines.first().unwrap().len();

        let mut pivoted: Vec<String> = Vec::new();
        for x in 0..width {
            pivoted.push(lines.iter().map(|line| line.chars().nth(x).unwrap()).collect::<String>());
        }

        pivoted
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

    ws.part1_problems.iter().map(|p| p.solve()).sum::<i64>().to_string()
}

pub fn part2() -> String {
    let ws = MathWorksheet::from(default_input());

    ws.part2_problems.iter().map(|p| p.solve()).sum::<i64>().to_string()
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

        assert_eq!(ws.part1_problems.len(), 4);
        assert_eq!(ws.part1_problems[0], MathProblem { numbers: vec![123, 45, 6], operator: MathOperator::Multiply });
        assert_eq!(ws.part1_problems[1], MathProblem { numbers: vec![328, 64, 98], operator: MathOperator::Add });
        assert_eq!(ws.part1_problems[2], MathProblem { numbers: vec![51, 387, 215], operator: MathOperator::Multiply });
        assert_eq!(ws.part1_problems[3], MathProblem { numbers: vec![64, 23, 314], operator: MathOperator::Add });

        assert_eq!(ws.part2_problems.len(), 4);
        assert_eq!(ws.part2_problems[0], MathProblem { numbers: vec![1, 24, 356], operator: MathOperator::Multiply });
        assert_eq!(ws.part2_problems[1], MathProblem { numbers: vec![369, 248, 8], operator: MathOperator::Add });
        assert_eq!(ws.part2_problems[2], MathProblem { numbers: vec![32, 581, 175], operator: MathOperator::Multiply });
        assert_eq!(ws.part2_problems[3], MathProblem { numbers: vec![623, 431, 4], operator: MathOperator::Add });
    }

    #[test]
    fn test_worksheet_solve() {
        let input = "123 328  51 64 \r\n 45 64  387 23 \r\n  6 98  215 314\r\n*   +   *   +  ";
        let ws = MathWorksheet::from(input);
        assert_eq!(ws.part1_problems[0].solve(), 123 * 45 * 6);
        assert_eq!(ws.part1_problems[1].solve(), 328 + 64 + 98);
        assert_eq!(ws.part1_problems[2].solve(), 51 * 387 * 215);
        assert_eq!(ws.part1_problems[3].solve(), 64 + 23 + 314);

        assert_eq!(ws.part2_problems[0].solve(), 24 * 356);
        assert_eq!(ws.part2_problems[1].solve(), 369 + 248 + 8);
        assert_eq!(ws.part2_problems[2].solve(), 32 * 581 * 175);
        assert_eq!(ws.part2_problems[3].solve(), 623 + 431 + 4);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "7644505810277");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "12841228084455");
    }
}
