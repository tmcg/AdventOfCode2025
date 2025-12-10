

use advent::*;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
    circ: Option<i64>,
}

struct Playground {
    boxes: HashMap<(i64,i64,i64), JunctionBox>,
    part1_product: i64,
    part2_product: i64,
}

impl From<&str> for Playground {
    fn from(s: &str) -> Self {
        let lines = input_as_lines(s);
        let mut boxes = HashMap::new();

        for line in lines {
            let jbox = JunctionBox::from(line.as_str());
            boxes.insert((jbox.x, jbox.y, jbox.z), jbox);
        }

        Playground { boxes, part1_product: 0, part2_product: 0 }
    }
}

impl From<&str> for JunctionBox {
    fn from(s: &str) -> Self {
        let tokens = s.split(",").collect::<Vec<_>>();
        let x = tokens[0].parse::<i64>().unwrap();
        let y = tokens[1].parse::<i64>().unwrap();
        let z = tokens[2].parse::<i64>().unwrap();

        JunctionBox { x, y, z, circ: None }
    }
}

impl Playground {

    fn find_circuits(&mut self, conn_max: usize) {
        let mut next_id: i64 = 1;
        let mut conn_count: usize = 0;

        let mut combos = self.find_combos().into_iter();

        while conn_count < conn_max {
            //println!("conn_count = {}", conn_count);

            if let Some(combo) = combos.next() {
                let b1_key = &combo.0.hkey();
                let b2_key = &combo.1.hkey();
                let b1 = self.boxes.get(b1_key).unwrap();
                let b2 = self.boxes.get(b2_key).unwrap();

                //println!("b1={:?}  b2={:?}", b1_key, b2_key);

                if b1.circ != b2.circ || b1.circ.is_none() {
                    if b1.circ.is_none() && b2.circ.is_none() {
                        //println!("Creating new circuit: {:?}", next_id);
                        self.update_circuit(b1_key, Some(next_id));
                        self.update_circuit(b2_key, Some(next_id));
                        next_id += 1;
                    } else if b1.circ.is_some() && b2.circ.is_none()  {
                        //println!("Updating circuit: {:?} with {:?}", b1.circ, b2_key);
                        self.update_circuit(b2_key, b1.circ);
                    } else if b2.circ.is_some() && b1.circ.is_none() {
                        //println!("Updating circuit: {:?} with {:?}", b2.circ, b1_key);
                        self.update_circuit(b1_key, b2.circ);
                    } else {
                        //println!("Combining circuits: {:?} and {:?}", b1.circ, b2.circ);
                        self.combine_circuits(b1.circ.unwrap(), b2.circ.unwrap());
                    }
                }
                conn_count += 1;

                if self.is_single_circuit() {
                    //println!("Single circuit, breaking!");
                    self.part2_product = b1_key.0 * b2_key.0;
                    break;
                }
            } else {
                panic!("Exceeded iterator!");
            }
        }

        //self.print_combos();
        //self.print_circuits();

        self.part1_product = self.circuit_product_part1();
    }

    fn circuit_product_part1(&self) -> i64 {
        let circuit_ids = self.boxes.iter().filter_map(|b| b.1.circ).collect::<Vec<_>>();
        let mut circuit_groups: HashMap<i64, i64> = HashMap::new();

        for circuit_id in circuit_ids {
            let entry = circuit_groups.entry(circuit_id).or_insert(0);
            *entry += 1;
            //println!("circuit_id={}  count={}", circuit_id, entry);
        }

        circuit_groups.values().sorted_by(|v1, v2| Ord::cmp(v2, v1)).take(3).product::<i64>()
    }

    fn is_single_circuit(&self) -> bool {
        if let Some(k) = self.boxes.keys().take(1).next() {
            let c1 = self.boxes.get(k).unwrap().circ;

            return c1.is_some() && self.boxes.iter().all(|b| b.1.circ == c1);
        }
        false
    }

    fn combine_circuits(&mut self, ckeep: i64, cfold: i64) {
        self.boxes.iter_mut().for_each(|b| {
            if let Some(c) = b.1.circ && c == cfold {
                b.1.circ = Some(ckeep)
            }
        })
    }

    fn update_circuit(&mut self, key: &(i64,i64,i64), circ: Option<i64>) {
        if let Some(val) = self.boxes.get_mut(key) {
            val.circ = circ;
        }
    }

    fn find_combos(&self) -> Vec<(JunctionBox, JunctionBox, f64)> {

        let jb1 = self.boxes.iter().map(|x| x.1).collect::<Vec<_>>();
        jb1.iter()
            .cartesian_product(jb1.iter())
            .filter(|(b1, b2)| (b1.x != b2.x || b1.y != b2.y || b1.z != b2.z) && b1.ge(b2))
            .map(|(b1, b2)| {
                let d = b1.dist(b2);
                if b1 < b2 { (**b1, **b2, d) } else { (**b2, **b1, d) }
            })
            .sorted_by(|b1, b2| b1.2.total_cmp(&b2.2))
            .collect::<Vec<_>>()
    }

    #[allow(dead_code)]
    fn print_circuits(&self) {
        let boxes = self.boxes.iter().map(|b| b.1).collect::<Vec<&JunctionBox>>();
        boxes.iter().sorted_by(|a, b| Ord::cmp(&a.circ, &b.circ)).for_each(|f| println!("{:?}", f));
    }

    #[allow(dead_code)]
    fn print_combos(&self) {
        self.find_combos().iter().for_each(|f| println!("{:?}", f));
    }
}

impl JunctionBox {
    fn dist(&self, j: &JunctionBox) -> f64 {
        let p = &[self.x as f64, self.y as f64, self.z as f64];
        let q = &[j.x as f64, j.y as f64, j.z as f64];
        p.iter().zip(q.iter()).map(|(&x1, &x2)| (x2 - x1).powf(2.0)).sum::<f64>().sqrt()
    }

    fn hkey(&self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }
}

fn default_input() -> &'static str {
    include_input!(08)
}

fn sample_input() -> &'static str {
    concat!(
    "162,817,812\r\n",
    "57,618,57\r\n",
    "906,360,560\r\n",
    "592,479,940\r\n",
    "352,342,300\r\n",
    "466,668,158\r\n",
    "542,29,236\r\n",
    "431,825,988\r\n",
    "739,650,466\r\n",
    "52,470,668\r\n",
    "216,146,977\r\n",
    "819,987,18\r\n",
    "117,168,530\r\n",
    "805,96,715\r\n",
    "346,949,466\r\n",
    "970,615,88\r\n",
    "941,993,340\r\n",
    "862,61,35\r\n",
    "984,92,344\r\n",
    "425,690,689"
    )
}

pub fn part1() -> String {
    let mut pg = Playground::from(default_input());
    pg.find_circuits(1000);
    pg.part1_product.to_string()
}

pub fn part2() -> String {
    let mut pg = Playground::from(default_input());
    pg.find_circuits(10000000);
    pg.part2_product.to_string()
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
    fn test_input_from() {
        let input = "1,2,3\r\n4,5,6";
        let model = Playground::from(input);

        assert_eq!(model.boxes.len(), 2);
        let k1: (i64,i64,i64) = (1,2,3);
        let k2: (i64,i64,i64) = (4,5,6);
        assert_eq!(model.boxes.get(&k1).map(|f| f.x), Some(1));
        assert_eq!(model.boxes.get(&k1).map(|f| f.y), Some(2));
        assert_eq!(model.boxes.get(&k1).map(|f| f.z), Some(3));
        assert_eq!(model.boxes.get(&k2).map(|f| f.x), Some(4));
        assert_eq!(model.boxes.get(&k2).map(|f| f.y), Some(5));
        assert_eq!(model.boxes.get(&k2).map(|f| f.z), Some(6));
    }

    #[test]
    fn test_jbox_dist() {
        let j1 = JunctionBox { x: 162, y: 817, z: 812, circ: None };
        let j2 = JunctionBox { x: 431, y: 825, z: 988, circ: None };

        let jd1 = j1.dist(&j2);
        let jd2 = j2.dist(&j1);
        assert!(jd1 - jd1 < 0.001);
        assert!((321.56 - jd1).abs() < 0.001);
        assert!((321.56 - jd2).abs() < 0.001);
    }

    #[test]
    fn test_find_circuits() {
        let mut pg = Playground::from(sample_input());
        pg.find_circuits(10);

        assert_eq!(pg.circuit_product_part1(), 40);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "42315");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "8079278220");
    }
}
