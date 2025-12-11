
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
    //tiles_cw: bool,
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

        println!("tiles (len={})", tiles.len());

        let mut lines = Vec::new();

        for ia in 0..tiles.len() {
            let ib = (ia + 1) % tiles.len();
            let ta = tiles[ia];
            let tb = tiles[ib];
            lines.push((ta, tb, Theater::midpoint(&ta, &tb)));
        }

        Theater { tiles, lines }
    }
}

impl Theater {

    fn largest_rect_part1(&self) -> i64 {
        let cart = self.get_cart_product();
        //println!("{}",cart.len());
        cart.iter().map(|tx| tx.2).max().unwrap_or(0)
    }

    fn largest_rect_part2(&self) -> i64 {
        let mut cart = self.get_cart_product();

        cart.sort_by(|a, b| Ord::cmp(&b.2, &a.2));

        //println!("====DEBUGGING====");
        //println!("cart count={}", cart.len());
        //let vc = cart.iter().filter(|tx| self.valid_rect(tx)).count();
        //println!("valid count={}", vc);

        //for c in cart.iter() {
        //    println!("Rect ({},{}) to ({},{}) Area={}", c.0.x, c.0.y, c.1.x, c.1.y, c.2);
        //    let valid = self.valid_rect(&c);
        //    println!("Valid={}", valid);
        //}

        //println!("====CALCULATING====");
        //cart.iter().filter(|tx| self.valid_rect(tx)).map(|tx| tx.2).max().unwrap_or(0)

        for c in cart.iter() {
            if self.valid_rect(c.0, c.1) {
                return c.2;
            }
        }

        0
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

        let q1 = self.tiles.iter().any(|t| Theater::rect_contains(&r_min, &r_max, t));
        // rect contains red tile line midpoint?
        let q2 = self.lines.iter().any(|k| Theater::rect_contains(&r_min, &r_max, &k.2));

        /*
        // rect contains red tile?
        let area = (r_max.x - r_min.x + 1) * (r_max.y - r_min.y + 1);
        println!("Rect ({},{}) to ({},{})  area={} q1={} q2={} result={}", r_min.x, r_min.y, r_max.x, r_max.y, area, q1, q2, !q1 && !q2);
        if !q1 && !q2 {
            for mp in &self.lines {
                println!("midpoint of ({},{}) to ({},{}) = ({},{})", mp.0.x, mp.0.y, mp.1.x, mp.1.y, mp.2.x, mp.2.y);
            }
        }
        */

        !q1 && !q2
    }

    fn rect_contains(r_min: &Tile, r_max: &Tile, p: &Tile) -> bool {
        r_min.x < p.x && r_max.x > p.x && r_min.y < p.y && r_max.y > p.y
    }

    fn midpoint(t1: &Tile, t2: &Tile) -> Tile {
        if t1.x == t2.x {
            let y = (t2.y.max(t1.y) + t2.y.min(t1.y)) / 2;
            Tile { x: t1.x, y }
        } else {
            let x = (t2.x.max(t1.x) + t2.x.min(t1.x)) / 2;
            Tile { x, y: t1.y }
        }
    }

    /*
    fn valid_rect(&self, tx: &&(&Tile, &Tile, i64)) -> bool {
        let rx_min = tx.0.x.min(tx.1.x);
        let rx_max = tx.0.x.max(tx.1.x);
        let ry_min = tx.0.y.min(tx.1.y);
        let ry_max = tx.0.y.max(tx.1.y);
        println!("Rect rx_min={} ry_min={} rx_max={} ry_max={}", rx_min, ry_min, rx_max, ry_max);

        self.lines.iter().all(|line| {
            let p1 = line.0;
            let p2 = line.1;

            let mut result = false;
            let mut branch: &str = "ERROR";
            let px_min = p1.x.min(p2.x);
            let py_min = p1.y.min(p2.y);
            let px_max = p1.x.max(p2.x);
            let py_max = p1.y.max(p2.y);

            println!("  Line p1=({},{}) p2=({},{})  {}{}{}{}", p1.x, p1.y, p2.x, p2.y,
                if p1.x > p2.x { "Left" } else { "" },
                if p1.x < p2.x { "Right" } else { "" },
                if p1.y > p2.y { "Up" } else { "" },
                if p1.y < p2.y { "Down" } else { "" });
            println!("  Line px_min={} py_min={} px_max={} py_max={}", px_min, py_min, px_max, py_max);

            if p1.x == p2.x {
                // vertical boundary
                let line_up = p1.y > p2.y;
                let line_dn = p1.y < p2.y;

                if ry_min > (py_max - 1) || ry_max < (py_min + 1) {
                    // if rect doesn't overlap green, assume valid
                    //result = true;
                    branch = "Vert-A";
                } else if (self.tiles_cw && line_dn) || (!self.tiles_cw && line_up) {
                    // check rect is fully to the left
                    result = p1.x >= rx_max;
                    branch = "Vert-B";
                } else if (self.tiles_cw && line_up) || (!self.tiles_cw && line_dn) {
                    // check rect is fully to the right
                    result = p1.x <= rx_min;
                    branch = "Vert-C";
                }
            } else {
                // horizontal boundary
                let line_lt = p1.x > p2.x;
                let line_rt = p1.x < p2.x;

                if rx_min > (px_max - 1) || rx_max < (px_min + 1) {
                    // if rect doesn't overlap green, assume valid
                    //result = true;
                    branch = "Horz-A";
                } else if (self.tiles_cw && line_lt) || (!self.tiles_cw && line_rt) {
                    // check rect is fully above
                    result = p1.y >= ry_max;
                    branch = "Horz-B";
                } else if (self.tiles_cw && line_rt) || (!self.tiles_cw && line_lt) {
                    // check rect is fully below
                    result = p1.y <= ry_min;
                    branch = "Horz-C";
                }
            }

            println!("    Result = {}  Path {}   ", result, branch);

            result
        })
    }

    fn tiles_clockwise(points: &[Tile]) -> bool {
        let first = points.first().unwrap();
        let mut tiles = points.to_vec();
        tiles.push(*first);

        let n = tiles.len();
        if n < 3 {
            panic!("degenerate area");
        }

        let mut area2: i128 = 0;

        for i in 0..n {
            let p1 = tiles[i];
            let p2 = tiles[(i + 1) % n];
            area2 += (p1.x as i128) * (p2.y as i128) - (p2.x as i128) * (p1.y as i128);
        }

        let area = (area2 as f64) * 0.5;
        
        // y increases downward, positive area is clockwise
        area > 0.0
    }
    */
}

impl Tile {
    fn rect_size(&self, t0: &Tile) -> i64 {
        let dx = self.x.max(t0.x) - self.x.min(t0.x) + 1;
        let dy = self.y.max(t0.y) - self.y.min(t0.y) + 1;
        dx * dy
    }
}

/*
fn rect_crosses(r1: &Tile, r2: &Tile, b1: &Tile, b2: &Tile) -> bool {
    let rx_min = r1.x.min(r2.x);
    let rx_max = r1.x.max(r2.x);
    let ry_min = r1.y.min(r2.y);
    let ry_max = r1.y.max(r2.y);

    //println!("min={},{} max={},{}", rx_min, ry_min, rx_max, ry_max);
    //println!("b1.y={} b2.y= {}", b1.y, b2.y);

    if b1.y == b2.y {
        // Boundary is horizontal
        let bx_min = b1.x.min(b2.x);
        let bx_max = b1.x.max(b2.x);
        //println!("horiz: bx_min={},bx_max{}", bx_min, bx_max);
        (rx_min >= bx_min && rx_min <= bx_max && ry_min < b1.y && ry_max > b1.y) ||
        (rx_max >= bx_min && rx_max <= bx_max && ry_min < b1.y && ry_max > b1.y)
    } else {
        // Boundary is vertical
        let by_min = b1.y.min(b2.y);
        let by_max = b1.y.max(b2.y);
        //println!("vert: by_min={}, by_max{}", by_min, by_max);
        (ry_min >= by_min && ry_min <= by_max && rx_min < b1.x && rx_max > b1.x) ||
        (ry_max >= by_min && ry_max <= by_max && rx_min < b1.x && rx_max > b1.x)
    }
}

fn intersect(a1: &Tile, a2: &Tile, b1: &Tile, b2: &Tile) -> bool {
    let ah = a1.y == a2.y;
    let bh = b1.y == b2.y;
    let ax_min = a1.x.min(a2.x);
    let ax_max = a1.x.max(a2.x);
    let ay_min = a1.y.min(a2.y);
    let ay_max = a1.y.max(a2.y);
    let bx_min = b1.x.min(b2.x);
    let bx_max = b1.x.max(b2.x);
    let by_min = b1.y.min(b2.y);
    let by_max = b1.y.max(b2.y);

    (ah && (ah != bh) && (
        b1.x >= ax_min
        && b1.x <= ax_max
        && a1.y >= by_min
        && a1.y <= by_max
    )) ||
    (bh && ah != bh && (
        a1.x >= bx_min
        && a1.x <= bx_max
        && b1.y >= ay_min
        && b1.y <= ay_max
    )) ||
    (ah && bh && (
        a1.y == b1.y
        && ax_min <= bx_max
        && bx_min <= ax_max
    )) ||
    (!ah && !bh && (
        a1.x == b1.x
        && ay_min <= by_max
        && by_min <= ay_max
    ))
}
*/

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
    //String::from("zz")
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
        //1565730054 too low
        assert_eq!(part2(), "1573359081");
    }
}