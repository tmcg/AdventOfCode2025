
use advent::*;
use itertools::Itertools;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
struct RedTile {
    x: i64,
    y: i64,
}

struct Theater {
    tiles: Vec<RedTile>,
}

impl From<&str> for RedTile {
    fn from(s: &str) -> Self {
        let tokens = s.split(",").collect::<Vec<_>>();
        let x = tokens[0].parse::<i64>().unwrap();
        let y = tokens[1].parse::<i64>().unwrap();

        RedTile { x, y }
    }
}

impl From<&str> for Theater {
    fn from(s: &str) -> Self {
        let tiles = input_as_lines(s).iter()
            .map(|x| RedTile::from(x.as_str()))
            .collect::<Vec<_>>();

        Theater { tiles }
    }
}

impl Theater {
    fn largest_rect(&self) -> i64 {

        let cart = self.tiles.iter()
            .cartesian_product(self.tiles.iter())
            .filter(|(t1, t2)| (t1.x != t2.x || t1.y != t2.y) && t1.ge(t2))
            .map(|tx| (tx.0, tx.1, tx.0.rect_size(tx.1)))
            .collect::<Vec<_>>();

        println!("{}",cart.len());

        cart.iter().map(|tx| tx.2).max().unwrap_or(0)
    }
}

impl RedTile {
    fn rect_size(&self, t0: &RedTile) -> i64 {
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

pub fn part1() -> String {
    let th = Theater::from(default_input());
    th.largest_rect().to_string()
}

pub fn part2() -> String {
    //let th = Theater::from(default_input());
    //th.tiles.len().to_string()
    String::from("aa")
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
    fn test_redtile_from() {
        let tile = RedTile::from("34,56");
        assert_eq!(tile.x, 34);
        assert_eq!(tile.y, 56);
    }

    #[test]
    fn test_theater_from() {
        let th = Theater::from("34,56\r\n67,89");
        assert_eq!(th.tiles.len(), 2);
        assert_eq!(th.tiles[0].x, 34);
        assert_eq!(th.tiles[0].y, 56);
        assert_eq!(th.tiles[1].x, 67);
        assert_eq!(th.tiles[1].y, 89);
    }

    #[test]
    fn test_theater_largest_rect() {
        let th = Theater::from(sample_input());
        assert_eq!(th.largest_rect(), 50);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "4781546175");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "zz");
    }
}
