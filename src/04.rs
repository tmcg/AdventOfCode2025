
use advent::*;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
enum TileContent {
    #[default]
    Empty,
    Roll,
}

#[derive(Debug, Default, Clone)]
struct Tile {
    content: TileContent,
}

#[derive(Debug, Default, Clone)]
struct Warehouse {
    tiles: HashMap<Point32, Tile>,
    width: i32,
    height: i32,
}

impl Warehouse {
    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.get(&Point32 { x, y })
    }

    fn get_tile_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        self.tiles.get_mut(&Point32 { x, y })
    }

    fn remove_roll(&mut self, x: i32, y: i32) {
        if let Some(tile) = self.get_tile_mut(x, y) {
            tile.content = TileContent::Empty;
        }
    }

    fn is_roll(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y).is_some_and(|t| t.content == TileContent::Roll)
    }

    fn is_accessible(&self, x: i32, y: i32) -> bool {
        let mut n = 0;
        if self.is_roll(x - 1, y - 1) { n += 1; }
        if self.is_roll(x - 1, y) { n += 1; }
        if self.is_roll(x - 1, y + 1) { n += 1; }
        if self.is_roll(x, y - 1) { n += 1; }
        if self.is_roll(x, y + 1) { n += 1; }
        if self.is_roll(x + 1, y) { n += 1; }
        if self.is_roll(x + 1, y - 1) { n += 1; }
        if self.is_roll(x + 1, y + 1) { n += 1; }

        n < 4
    }

    fn find_accessible_rolls(&self) -> Vec<Point32> {
        let mut accessible_rolls = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_roll(x, y) && self.is_accessible(x, y) {
                    accessible_rolls.push(Point32 { x, y });
                }
            }
        }

        accessible_rolls
    }

    fn remove_accessible_rolls(&mut self) -> Vec<Point32> {
        let mut accessible_rolls = Vec::new();
        let mut more = true;

        while more {
            let current_rolls = self.find_accessible_rolls();
            for p in current_rolls.iter() {
                self.remove_roll(p.x, p.y);
            }

            more = !current_rolls.is_empty();
            accessible_rolls = [accessible_rolls, current_rolls].concat();
        }

        accessible_rolls
    }
}

impl From<&str> for Warehouse {
    fn from(s: &str) -> Self {
        let lines = input_as_lines(s);

        let height = lines.len() as i32;
        let width = lines[0].len() as i32;
        let mut wh = Warehouse { width, height, ..Default::default() };

        for (ey, line) in lines.iter().enumerate() {
            let y = ey as i32;
            for (ex, c) in line.chars().enumerate() {
                let x = ex as i32;

                let content = match c {
                    '.' => TileContent::Empty,
                    '@' => TileContent::Roll,
                    _ => continue,
                };

                wh.tiles.insert(Point32 { x, y }, Tile { content });
            }
        }

        wh
    }
}

fn default_input() -> &'static str {
    include_input!(04)
}

pub fn part1() -> String {
    let wh = Warehouse::from(default_input());
    wh.find_accessible_rolls().len().to_string()
}

pub fn part2() -> String {
    let mut wh = Warehouse::from(default_input());
    wh.remove_accessible_rolls().len().to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "..@@.@@@@.\r\n@@@.@.@.@@\r\n@@@@@.@.@@\r\n@.@@@@..@.\r\n@@.@@@@.@@\r\n.@@@@@@@.@\r\n.@.@.@.@@@\r\n@.@@@.@@@@\r\n.@@@@@@@@.\r\n@.@.@@@.@."
    }

    #[test]
    fn test_warehouse_from() {
        let wh = Warehouse::from(sample_input());

        assert_eq!(wh.width, 10);
        assert_eq!(wh.height, 10);
        assert_eq!(wh.tiles.len(), 100);

        assert_tile_content(wh.get_tile(-1, -1), TileContent::Empty);
        assert_tile_content(wh.get_tile(0, 0), TileContent::Empty);
        assert_tile_content(wh.get_tile(1, 0), TileContent::Empty);
        assert_tile_content(wh.get_tile(2, 0), TileContent::Roll);
        assert_tile_content(wh.get_tile(3, 0), TileContent::Roll);
    }

    fn assert_tile_content(tile: Option<&Tile>, expected: TileContent) {
        if let Some(t) = &tile {
            assert_eq!(t.content, expected);
        } else {
            assert_eq!(TileContent::Empty, expected);
        }
    }

    #[test]
    fn test_is_accessible() {
        let wh = Warehouse::from(sample_input());

        let assert_accessible_roll = |x, y, expected| assert_eq!(wh.is_roll(x, y) && wh.is_accessible(x, y), expected);

        assert_accessible_roll(0, 0, false);
        assert_accessible_roll(1, 0, false);
        assert_accessible_roll(2, 0, true);
        assert_accessible_roll(3, 0, true);
        assert_accessible_roll(4, 0, false);
        assert_accessible_roll(5, 0, true);
        assert_accessible_roll(6, 0, true);
    }

    #[test]
    fn test_find_accessible_rolls() {
        let wh = Warehouse::from(sample_input());
        assert_eq!(wh.find_accessible_rolls().len(), 13);
    }

    #[test]
    fn test_remove_accessible_rolls() {
        let mut wh = Warehouse::from(sample_input());
        assert_eq!(wh.remove_accessible_rolls().len(), 43);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "1602");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "9518");
    }
}
