
use advent::*;

type IngredientId = i64;

#[derive(Debug, PartialEq, Eq)]
struct IngredientItem {
    id: IngredientId,
}

#[derive(Debug, PartialEq, Eq)]
struct IngredientRange {
    start: IngredientId,
    end: IngredientId,
}

struct Inventory {
    fresh: Vec<IngredientRange>,
    items : Vec<IngredientItem>,
}

impl From<&str> for Inventory {
    fn from(s: &str) -> Self {
        let lines = input_as_lines(s).into_iter().filter(|x| !x.is_empty()).collect::<Vec<_>>();

        let fresh = lines.iter().filter(|x| x.contains("-")).map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<IngredientId>().unwrap();
            let end = parts[1].parse::<IngredientId>().unwrap();
            IngredientRange { start, end }
        }).collect();

        let items = lines.iter().filter(|x| !x.contains("-")).map(|line| {
            let id = line.parse::<IngredientId>().unwrap();
            IngredientItem { id }
        }).collect();

        Inventory { fresh, items }
    }
}

impl IngredientRange {
    fn contains(&self, item: &IngredientItem) -> bool {
        item.id >= self.start && item.id <= self.end
    }
}

impl Inventory {
    fn count_fresh_items(&self) -> i64 {
        self.items.iter().filter(|item| {
            self.fresh.iter().any(|range| range.contains(item))
        }).count() as i64
    }
}

fn default_input() -> &'static str {
    include_input!(05)
}

pub fn part1() -> String {
    let inv = Inventory::from(default_input());
    inv.count_fresh_items().to_string()
}

pub fn part2() -> String {
    let inv = Inventory::from(default_input());
    inv.fresh.len().to_string()
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
        let input = "3-5\r\n10-14\r\n16-20\r\n12-18\r\n\r\n1\r\n5\r\n8\r\n11\r\n17\r\n32";
        let inv = Inventory::from(input);
        assert_eq!(inv.fresh.len(), 4);
        assert_eq!(inv.items.len(), 6);
        assert_eq!(inv.fresh[0], IngredientRange { start: 3, end: 5 });
        assert_eq!(inv.fresh[1], IngredientRange { start: 10, end: 14 });
        assert_eq!(inv.fresh[2], IngredientRange { start: 16, end: 20 });
        assert_eq!(inv.fresh[3], IngredientRange { start: 12, end: 18 });
        assert_eq!(inv.items[0], IngredientItem { id: 1 });
        assert_eq!(inv.items[1], IngredientItem { id: 5 });
        assert_eq!(inv.items[2], IngredientItem { id: 8 });
        assert_eq!(inv.items[3], IngredientItem { id: 11 });
        assert_eq!(inv.items[4], IngredientItem { id: 17 });
        assert_eq!(inv.items[5], IngredientItem { id: 32 });
    }

    #[test]
    fn test_count_fresh_items() {
        let input = "3-5\r\n10-14\r\n16-20\r\n12-18\r\n\r\n1\r\n5\r\n8\r\n11\r\n17\r\n32";
        let inv = Inventory::from(input);
        let count = inv.count_fresh_items();
        assert_eq!(count, 3); // items 5, 11, and 17 are fresh
    }   

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "739");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "182");
    }
}
