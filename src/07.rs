
use advent::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct TachyonManifold {
    grid: HashMap<(i64, i64), TachyonCell>,
    width: i64,
    height: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TachyonCell {
    cell_type: TachyonCellType
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TachyonCellType {
    Empty,
    Origin,
    Splitter,
    Beam,
}

impl From<char> for TachyonCellType {
    fn from(ch: char) -> Self {
        match ch {
            'S' => TachyonCellType::Origin,
            '^' => TachyonCellType::Splitter,
            '|' => TachyonCellType::Beam,
            _ => TachyonCellType::Empty,
        }
    }
}

impl TachyonManifold {

    fn fire_beam(&mut self) -> i64 {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.cell(x, y).unwrap();
                let under_beam = self.is_cell_type(x, y - 1, TachyonCellType::Beam);
                let under_orig = self.is_cell_type(x, y - 1, TachyonCellType::Origin);

                match c.cell_type {
                    TachyonCellType::Splitter => {
                        if under_orig || under_beam {
                            self.split_beam(x, y);
                        }
                    },
                    TachyonCellType::Empty => {
                        if under_orig || under_beam {
                            self.extend_beam(x, y);
                        }
                    }
                    _ => {}
                }
            }
        }

        self.count_splits()
    }

    fn split_beam(&mut self, x: i64, y: i64) {
        if let Some(lc) = self.cell_mut(x - 1, y) && lc.cell_type == TachyonCellType::Empty {
            lc.cell_type = TachyonCellType::Beam;
        }
        if let Some(rc) = self.cell_mut(x + 1, y) && rc.cell_type == TachyonCellType::Empty {
            rc.cell_type = TachyonCellType::Beam;
        }
    }

    fn extend_beam(&mut self, x: i64, y: i64) {
        if let Some(c) = self.cell_mut(x, y) && c.cell_type == TachyonCellType::Empty {
            c.cell_type = TachyonCellType::Beam;
        }
    }

    fn count_splits(&self) -> i64 {
        let mut split_count = 0;

        for y in 1..self.height {
            for x in 0..self.width {
                let c = self.cell(x, y).unwrap();
                let u = self.cell(x, y - 1).unwrap();

                if c.cell_type == TachyonCellType::Splitter && u.cell_type == TachyonCellType::Beam {
                    split_count += 1;
                }
            }
        }

        split_count
    }

    fn cell(&self, x: i64, y: i64) -> Option<&TachyonCell> {
        self.grid.get(&(x, y))
    }

    fn cell_mut(&mut self, x: i64, y: i64) -> Option<&mut TachyonCell> {
        self.grid.get_mut(&(x, y))
    }

    fn is_cell_type(&self, x: i64, y: i64, t: TachyonCellType) -> bool {
        match self.cell(x, y) {
            Some(c) => c.cell_type == t,
            None => false,
        }
    }

    /*
    fn is_empty(&self, x: i64, y: i64) -> bool {
        self.is_cell_type(x, y, TachyonCellType::Empty)
    }

    fn is_origin(&self, x: i64, y: i64) -> bool {
        self.is_cell_type(x, y, TachyonCellType::Origin)
    }

    fn is_splitter(&self, x: i64, y: i64) -> bool {
        self.is_cell_type(x, y, TachyonCellType::Splitter)
    }

    fn is_beam(&self, x: i64, y: i64) -> bool {
        self.is_cell_type(x, y, TachyonCellType::Beam)
    }

    fn print(&self) {
        println!("w={},h={}", self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(c) = self.cell(x, y) {
                    print!("{}", match c.cell_type {
                        TachyonCellType::Origin => "S",
                        TachyonCellType::Beam => "|",
                        TachyonCellType::Splitter => "^",
                        _ => "."
                    });
                }
            }
            println!()
        }
    }
    */
}

impl From<&str> for TachyonManifold {
    fn from(s: &str) -> Self {
        let input = input_as_grid(s);
        println!("{:?}", input.len());
        let mut grid: HashMap<(i64, i64), TachyonCell> = HashMap::new();
        let mut width: i64 = 0;
        let mut height: i64 = 0;

        input.iter().for_each(|(key, val)| {
            grid.insert(*key, TachyonCell { cell_type: TachyonCellType::from(*val) });
            width = width.max(key.0 + 1);
            height = height.max(key.1 + 1);
        });

        TachyonManifold { grid, width, height }
    }
}

fn default_input() -> &'static str {
    include_input!(07)
}

fn sample_input() -> &'static str {
    ".......S.......\r\n.......|.......\r\n.......^.......\r\n...............\r\n......^.^......\r\n...............\r\n.....^.^.^.....\r\n...............\r\n....^.^...^....\r\n...............\r\n...^.^...^.^...\r\n...............\r\n..^...^.....^..\r\n...............\r\n.^.^.^.^.^...^.\r\n..............."
}

pub fn part1() -> String {
    
    let mut mf = TachyonManifold::from(default_input());
    mf.fire_beam().to_string()
}

pub fn part2() -> String {
    //let model = TachyonManifold::from(default_input());
    //model.lines.len().to_string()
    String::from("zz")
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
    fn test_sample_input() {
        let mf = TachyonManifold::from(sample_input());
        assert_eq!(mf.cell(0, 0).unwrap().cell_type, TachyonCellType::Empty);
        assert_eq!(mf.cell(7, 0).unwrap().cell_type, TachyonCellType::Origin);
        assert_eq!(mf.cell(7, 1).unwrap().cell_type, TachyonCellType::Beam);
        assert_eq!(mf.cell(7, 2).unwrap().cell_type, TachyonCellType::Splitter);
        assert_eq!(mf.grid.len(), 240);
    }

    #[test]
    fn test_fire_beam() {
        let mut mf = TachyonManifold::from(sample_input());
        mf.fire_beam();
        assert_eq!(mf.cell(7, 0).unwrap().cell_type, TachyonCellType::Origin);
        assert_eq!(mf.cell(7, 1).unwrap().cell_type, TachyonCellType::Beam);
        assert_eq!(mf.cell(6, 2).unwrap().cell_type, TachyonCellType::Beam);
        assert_eq!(mf.cell(7, 2).unwrap().cell_type, TachyonCellType::Splitter);
        assert_eq!(mf.cell(8, 2).unwrap().cell_type, TachyonCellType::Beam);
        assert_eq!(mf.grid.len(), 240);
    }

    #[test]
    fn test_count_splits() {
        let mut mf = TachyonManifold::from(sample_input());
        assert_eq!(mf.fire_beam(), 21);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "1555");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "zz");
    }
}
