
use advent::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct TachyonManifold {
    grid: HashMap<(i64, i64), TachyonCell>,
    width: i64,
    height: i64,
    debug: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TachyonCell {
    cell_type: TachyonCellType,
    timelines: i64,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TachyonCellType {
    Empty,
    Origin,
    Splitter,
}

impl From<char> for TachyonCellType {
    fn from(ch: char) -> Self {
        match ch {
            'S' => TachyonCellType::Origin,
            '^' => TachyonCellType::Splitter,
            _ => TachyonCellType::Empty,
        }
    }
}

impl TachyonManifold {

    fn fire_beam(&mut self) -> (i64, i64) {
        let empty_cell = TachyonCell { cell_type: TachyonCellType::Empty, timelines: 0 };

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(c) = self.cell(x, y) && c.cell_type == TachyonCellType::Empty {
                    let u = self.cell(x, y - 1).unwrap_or(&empty_cell);
                    let lu = self.cell(x - 1, y - 1).unwrap_or(&empty_cell);
                    let ru = self.cell(x + 1, y - 1).unwrap_or(&empty_cell);
                    let lc = self.cell(x - 1, y).unwrap_or(&empty_cell);
                    let rc = self.cell(x + 1, y).unwrap_or(&empty_cell);

                    let left_add = if lc.cell_type == TachyonCellType::Splitter { lu.timelines } else { 0 };
                    let right_add = if rc.cell_type == TachyonCellType::Splitter { ru.timelines } else { 0 };
                    self.extend_beam(x, y, u.timelines + left_add + right_add);
                }
            }

            if self.debug { 
                let s = format!("(y={})", y);
                self.print_timelines(s.as_str());
                println!();
                self.print_cells("");
            }
        }

        let c1 = self.count_splits();
        let c2 = self.count_timelines();
        (c1, c2)
    }

    fn extend_beam(&mut self, x: i64, y: i64, timelines: i64) {
        if let Some(c) = self.cell_mut(x, y) {
            c.timelines += timelines;
        }
    }

    fn count_splits(&self) -> i64 {
        let mut split_count = 0;

        for y in 1..self.height {
            for x in 0..self.width {
                let c = self.cell(x, y).unwrap();
                let u = self.cell(x, y - 1).unwrap();

                if c.cell_type == TachyonCellType::Splitter && u.timelines > 0 {
                    split_count += 1;
                }
            }
        }

        split_count
    }

    fn count_timelines(&self) -> i64 {
        let fn_cell = |x| self.cell(x, self.height - 1).unwrap();
        (0..self.width).map(fn_cell).map(|c| c.timelines).sum::<i64>()
    }

    fn cell(&self, x: i64, y: i64) -> Option<&TachyonCell> {
        self.grid.get(&(x, y))
    }

    fn cell_mut(&mut self, x: i64, y: i64) -> Option<&mut TachyonCell> {
        self.grid.get_mut(&(x, y))
    }

    fn print<F>(&self, print_fn: F) where F: Fn(&TachyonCell) {
        //println!("w={},h={}", self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(c) = self.cell(x, y) {
                    print_fn(c);
                }
            }
            println!()
        }
    }

    fn print_cells(&self, name: &str) {
        println!();
        if !name.is_empty() {
            println!("==== {} ====", name);
        }
        
        self.print(|c| {
            print!("{}", match c.cell_type {
                TachyonCellType::Origin => "S",
                TachyonCellType::Splitter => "^",
                _ => if c.timelines > 0 { "|" } else { "." }
            });
        });
    }

    fn print_timelines(&self, name: &str) {
        println!();
        println!("==== {} ====", name);
        self.print(|c| {
            let s = if c.cell_type == TachyonCellType::Splitter {
                String::from("  ^")
            } else {
                format!("{:width$}", c.timelines, width=3)
            };
            
            print!("{}", s);
        });
    }
}

impl From<&str> for TachyonManifold {
    fn from(s: &str) -> Self {
        let input = input_as_grid(s);
        let mut grid: HashMap<(i64, i64), TachyonCell> = HashMap::new();
        let mut width: i64 = 0;
        let mut height: i64 = 0;

        input.iter().for_each(|(key, val)| {
            let cell_type = TachyonCellType::from(*val);
            let timelines = match cell_type {
                TachyonCellType::Origin => 1,
                _ => 0,
            };
            grid.insert(*key, TachyonCell { cell_type, timelines });
            width = width.max(key.0 + 1);
            height = height.max(key.1 + 1);
        });

        TachyonManifold { grid, width, height, debug: false }
    }
}

fn default_input() -> &'static str {
    include_input!(07)
}

fn sample_input() -> &'static str {
    include_input!(07a)
}

pub fn part1() -> String {
    let mut mf = TachyonManifold::from(default_input());
    mf.fire_beam().0.to_string()
}

pub fn part2() -> String {
    let mut mf = TachyonManifold::from(default_input());
    mf.fire_beam().1.to_string()
}

fn main() {
    let _ = sample_input();
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifold_from() {
        let mf = TachyonManifold::from(sample_input());
        assert_eq!(mf.cell(0, 0).unwrap().cell_type, TachyonCellType::Empty);
        assert_eq!(mf.cell(7, 0).unwrap().cell_type, TachyonCellType::Origin);
        assert_eq!(mf.cell(7, 1).unwrap().cell_type, TachyonCellType::Empty);
        assert_eq!(mf.cell(7, 2).unwrap().cell_type, TachyonCellType::Splitter);
        assert_eq!(mf.grid.len(), 240);
    }

    #[test]
    fn test_fire_beam() {
        let mut mf = TachyonManifold::from(sample_input());
        mf.fire_beam();
        assert_eq!(mf.cell(7, 0).unwrap().cell_type, TachyonCellType::Origin);
        assert_eq!(mf.cell(7, 1).unwrap().cell_type, TachyonCellType::Empty);
        assert_eq!(mf.cell(7, 1).unwrap().timelines, 1);
        assert_eq!(mf.cell(6, 2).unwrap().cell_type, TachyonCellType::Empty);
        assert_eq!(mf.cell(6, 2).unwrap().timelines, 1);
        assert_eq!(mf.cell(7, 2).unwrap().cell_type, TachyonCellType::Splitter);
        assert_eq!(mf.cell(8, 2).unwrap().cell_type, TachyonCellType::Empty);
        assert_eq!(mf.cell(8, 2).unwrap().timelines, 1);
        assert_eq!(mf.grid.len(), 240);
    }

    #[test]
    fn test_count_sample() {
        let mut mf = TachyonManifold::from(sample_input());
        let (sc, tc) = mf.fire_beam();
        assert_eq!(sc, 21);
        assert_eq!(tc, 40);
    }

    #[test]
    fn test_count_basic1() {
        let input = "..S..\r\n..|..\r\n..^..\r\n.^.^.\r\n.....";
        let mut mf = TachyonManifold::from(input);
        let (sc, tc) = mf.fire_beam();
        assert_eq!(sc, 3);
        assert_eq!(tc, 4);
    }
    
    #[test]
    fn test_count_basic2() {
        let input = "...S...\r\n.......\r\n...^...\r\n.......\r\n..^.^..\r\n.......\r\n.^.^.^.\r\n.......";
        let mut mf = TachyonManifold::from(input);
        let (_, tc) = mf.fire_beam();
        assert_eq!(tc, 8);
    }

    #[test]
    fn test_count_basic3() {
        let input = "...S...\r\n.......\r\n...^...\r\n.......\r\n..^....\r\n.......\r\n.^.^...\r\n.......";
        let mut mf = TachyonManifold::from(input);
        let (_, tc) = mf.fire_beam();
        assert_eq!(tc, 5);
    }

    #[test]
    fn test_count_basic4() {
        let input = "....S....\r\n.........\r\n....^....\r\n...^.^...\r\n..^...^..\r\n...^.^...\r\n....^....\r\n.........";
        let mut mf = TachyonManifold::from(input);
        let (_, tc) = mf.fire_beam();
        assert_eq!(tc, 12)
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "1555");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "12895232295789");
    }
}
