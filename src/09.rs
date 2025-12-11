
use advent::*;
use itertools::Itertools;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
struct Tile {
    x: i64,
    y: i64,
}

struct Theater {
    tiles: Vec<Tile>,
    lines: Vec<(Tile, Tile, Tile)>,
}

impl From<&str> for Tile {
    fn from(s: &str) -> Self {
        let tokens = s.split(",").collect::<Vec<_>>();
        let x = tokens[0].parse::<i64>().unwrap();
        let y = tokens[1].parse::<i64>().unwrap();

        Tile { x, y }
    }
}

impl From<&str> for Theater {
    fn from(s: &str) -> Self {
        let tiles = input_as_lines(s).iter()
            .map(|x| Tile::from(x.as_str()))
            .collect::<Vec<_>>();

        let lines = tiles.iter()
            .zip(tiles.iter().cycle().skip(1))
            .map(|(ta, tb)| (*ta, *tb, Theater::midpoint(ta, tb)))
            .take(tiles.len())
            .collect::<Vec<_>>();

        Theater { tiles, lines }
    }
}

impl Theater {

    fn largest_rect_part1(&self) -> i64 {
        self.get_cart_product().iter()
            .map(|c| c.2).max().unwrap_or(0)
    }

    fn largest_rect_part2(&self) -> i64 {
        self.get_cart_product().iter()
            .sorted_by(|a, b| Ord::cmp(&b.2, &a.2))
            .find(|c| self.valid_rect(c.0, c.1))
            .map(|c| c.2).unwrap_or(0)
    }

    fn get_cart_product(&self) -> Vec<(&Tile, &Tile, i64)> {
        self.tiles.iter()
            .cartesian_product(self.tiles.iter())
            .filter(|(t1, t2)| (t1.x != t2.x || t1.y != t2.y) && t1.ge(t2))
            .map(|tx| (tx.0, tx.1, tx.0.rect_size(tx.1)))
            .collect::<Vec<_>>()
    }

    fn valid_rect(&self, r1: &Tile, r2: &Tile) -> bool {
        let r_min = Tile { x: r1.x.min(r2.x), y: r1.y.min(r2.y) };
        let r_max = Tile { x: r1.x.max(r2.x), y: r1.y.max(r2.y) };

        // valid rects contain no red tile and no tile line midpoint
        !self.tiles.iter().any(|t| Theater::rect_contains(&r_min, &r_max, t)) &&
        !self.lines.iter().any(|k| Theater::rect_contains(&r_min, &r_max, &k.2))
    }

    fn rect_contains(r_min: &Tile, r_max: &Tile, p: &Tile) -> bool {
        r_min.x < p.x && r_max.x > p.x && r_min.y < p.y && r_max.y > p.y
    }

    fn midpoint(t1: &Tile, t2: &Tile) -> Tile {
        Tile { x: (t1.x + t2.x) / 2, y: (t1.y + t2.y) / 2 }
    }
}

impl Tile {
    fn rect_size(&self, t0: &Tile) -> i64 {
        let dx = self.x.max(t0.x) - self.x.min(t0.x) + 1;
        let dy = self.y.max(t0.y) - self.y.min(t0.y) + 1;
        dx * dy
    }
}

fn default_input() -> &'static str {
    include_input!(09)
}

fn sample_input() -> &'static str {
    concat!(
    "7,1\r\n",
    "11,1\r\n",
    "11,7\r\n",
    "9,7\r\n",
    "9,5\r\n",
    "2,5\r\n",
    "2,3\r\n",
    "7,3")
}

fn sample_input_b() -> &'static str {
    concat!(
    "5,1\r\n",
    "5,7\r\n",
    "2,7\r\n",
    "2,5\r\n",
    "4,5\r\n",
    "4,3\r\n",
    "2,3\r\n",
    "2,1")
}

pub fn part1() -> String {
    let th = Theater::from(default_input());
    th.largest_rect_part1().to_string()
}

pub fn part2() -> String {
    let th = Theater::from(default_input());
    th.largest_rect_part2().to_string()
}

fn main() {
    let _ = sample_input();
    let _ = sample_input_b();
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_from() {
        let tile = Tile::from("34,56");
        assert_eq!(tile.x, 34);
        assert_eq!(tile.y, 56);
    }

    #[test]
    fn test_theater_from() {
        let th = Theater::from("34,56\r\n67,89\r\n67,100");
        assert_eq!(th.tiles.len(), 3);
        assert_eq!(th.tiles[0].x, 34);
        assert_eq!(th.tiles[0].y, 56);
        assert_eq!(th.tiles[1].x, 67);
        assert_eq!(th.tiles[1].y, 89);
        assert_eq!(th.tiles[2].x, 67);
        assert_eq!(th.tiles[2].y, 100);

        assert_eq!(th.lines.len(), 3);
        assert_eq!(th.lines[0].0, Tile { x: 34, y: 56 });
        assert_eq!(th.lines[0].1, Tile { x: 67, y: 89 });
        assert_eq!(th.lines[1].0, Tile { x: 67, y: 89 });
        assert_eq!(th.lines[1].1, Tile { x: 67, y: 100 });
        assert_eq!(th.lines[2].0, Tile { x: 67, y: 100 });
        assert_eq!(th.lines[2].1, Tile { x: 34, y: 56 });
    }

    #[test]
    fn test_theater_midpoint() {
        let t1 = Tile { x: 2, y: 3 };
        let t2 = Tile { x: 7, y: 3 };
        let p = Tile { x: 4, y: 3};
        assert_eq!(Theater::midpoint(&t1, &t2), p)
    }

    #[test]
    fn test_theater_largest_rect_part1() {
        let th = Theater::from(sample_input());
        assert_eq!(th.largest_rect_part1(), 50);
    }

    #[test]
    fn test_theater_largest_rect_part2a() {
        let th = Theater::from(sample_input());
        assert_eq!(th.largest_rect_part2(), 24);
    }

    #[test]
    fn test_theater_largest_rect_part2b() {
        let th = Theater::from(sample_input_b());
        assert_eq!(th.largest_rect_part2(), 12);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "4781546175");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "1573359081");
    }
}