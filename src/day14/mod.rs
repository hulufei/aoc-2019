use std::collections::HashMap;
use std::ops::MulAssign;

const INPUT: &str = include_str!("./input");

#[derive(Debug, Clone, PartialEq)]
struct Val(u32, String);

impl Val {
    fn new(count: u32, name: &str) -> Self {
        Self(count, name.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Expr(Vec<Val>);

impl MulAssign<u32> for Expr {
    fn mul_assign(&mut self, n: u32) {
        for v in self.0.iter_mut() {
            let Val(count, name) = v;
            *v = Val::new(*count * n, &name);
        }
    }
}

impl Expr {
    fn new() -> Self {
        Self(vec![])
    }

    fn append(&mut self, other: Self) {
        self.0.append(&mut other.0.clone())
    }

    fn flatten(self) -> Self {
        let vals = self
            .0
            .into_iter()
            .fold(HashMap::new(), |mut acc, Val(num, name)| {
                *acc.entry(name).or_insert(0) += num;
                acc
            })
            .into_iter()
            .map(|(name, num)| Val::new(num, &name))
            .collect();

        Expr(vals)
    }
}

#[derive(Debug, PartialEq)]
struct Reactions {
    equations: HashMap<String, (u32, Expr)>,
}

impl Reactions {
    fn new(input: &str) -> Self {
        let parse_val = |v: &str| -> Val {
            let mut pair = v.trim().split(' ');
            let num = pair.next().and_then(|n| n.parse().ok()).unwrap();
            let name = pair.next().unwrap().to_owned();
            Val(num, name)
        };

        let equations = input
            .lines()
            .map(|line| {
                let mut reactions = line.split("=>");
                let left = reactions.next().unwrap();
                let right = reactions.next().unwrap();
                let Val(num, name) = parse_val(right);
                let expr = Expr(left.split(',').map(parse_val).collect());

                (name, (num, expr))
            })
            .collect();

        Self { equations }
    }

    fn expand_val(&self, val: Val) -> Expr {
        let Val(count, name) = val;
        println!("for ({}, {})", name, count);
        let (min_required_unit, expr) = self.equations.get(&name).unwrap();
        let multiply = (count as f32 / *min_required_unit as f32).ceil() as u32;
        let mut expr = expr.clone();
        // println!(
        //     "min requires {}, total requires {}",
        //     min_required_unit, multiply
        // );
        expr *= multiply;
        println!("expaned expr {:?}", expr);
        expr
    }

    fn calc_ore(&self) -> u32 {
        let mut requires = self.expand_val(Val::new(1, "FUEL"));
        loop {
            let mut expr = Expr::new();
            for val in requires.0 {
                match val {
                    Val(_, name) if name == "ORE" => continue,
                    _ => {
                        expr.append(self.expand_val(val));
                    }
                }
            }
            requires = expr.flatten();
            println!("requires expr {:?}", requires);
            match requires.0.as_slice() {
                [Val(v, name)] if name == "ORE" => return *v,
                _ => (),
            }
        }
    }
}

pub fn part_1() -> u32 {
    Reactions::new(INPUT).calc_ore()
}

#[test]
fn test_ore_calc() {
    let mut input = r"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

    assert_eq!(Reactions::new(input).calc_ore(), 165);

    input = r"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    assert_eq!(Reactions::new(input).calc_ore(), 13312);
}
