use crate::Part;

use core::iter::{
    Enumerate,
    Peekable,
};
use std::{
    str::Chars,
    collections::HashMap,
};



pub(crate) fn solve(input: Box<dyn Iterator<Item = String>>, part: Part) -> String {

    let mut nums: Vec<Number> = Vec::new();
    let mut syms: HashMap<Coord, Symbol> = HashMap::new();

    input.enumerate().for_each(|(y, line)| {
        let mut char_enumerator = line.chars().enumerate().peekable();
        loop {
            let (x, c) = match char_enumerator.peek() {
                Some((x_val, c)) => (*x_val, *c),
                None => break
            };

            match c {
                '.' => (),
                '\n' => (),
                _ if c.is_ascii_digit() => {
                    let num = parse_num_from(&mut char_enumerator.by_ref());
                        nums.push(Number { 
                            val: num,
                            origin: Coord { x: x as u32, y: y as u32 }, 
                            end: Coord { x: x as u32 + len_of(num), y: y as u32 }, 
                        });
                    continue; // already consumed the value, just continue now
                }
                '*' => {
                    syms.insert(Coord { x: x as u32, y: y as u32 }, Symbol::Cog);
                },
                _ => {
                    syms.insert(Coord { x: x as u32, y: y as u32 }, Symbol::NotPeriod);
                },
            };
            
            char_enumerator.next();
        }
    });

    let schematic = EngineSchematic::from(nums, syms);

    let total = match part {
        Part::Part1 => schematic.summed_part_numbers(),
        Part::Part2 => schematic.summed_gears_ratios(),
    };

    format!("{}", total)
}

fn parse_num_from<'a>(enumerator: &mut Peekable<Enumerate<Chars<'a>>>) -> u32 {
    let mut num_chars: Vec<char> = Vec::new();
    loop {
        match enumerator.peek() {
            Some((_, c)) if c.is_ascii_digit() => {
                num_chars.push(*c);
                enumerator.next(); // ensure we consume the value
            },
            _ => { break },
        }
    }

    num_chars.iter().collect::<String>().parse::<u32>().expect("digits should parse")
}

fn len_of(num: u32) -> u32 {
    format!("{num}").len() as u32 - 1
}

#[derive(Debug)]
struct EngineSchematic {
    part_numbers: Vec<Number>,
    symbols: HashMap<Coord, Symbol>,
}

impl EngineSchematic {
    fn from(numbers: Vec<Number>, symbols: HashMap<Coord, Symbol>) -> EngineSchematic {
        let valid_parts: Vec<Number> = numbers.iter()
            .filter_map(|num|{
            let mut sym_coords = symbols.keys();
            if sym_coords.any(|coord| num.is_adjacent_to(coord)) {
                Some(*num)
            } else {
                None
            }}).collect();

        EngineSchematic { part_numbers: valid_parts, symbols }
    }

    fn summed_part_numbers(&self) -> u32 {
        self.part_numbers.iter().map(|n| n.val).sum()
    }

    fn summed_gears_ratios(&self) -> u32 {
        let mut ratio_acc = 0u32;
        // let cogs: Vec<Coord> = 
        self.symbols.iter()
            .filter_map(|(c, s)| {
                match s {
                    Symbol::Cog => Some(c),
                    _ => None,
                }
            })
            .for_each(|c| {
                let adjacent_parts: Vec<&Number> = self.part_numbers.iter().filter(|n| n.is_adjacent_to(c)).collect();

                if adjacent_parts.len() == 2 {
                    ratio_acc += adjacent_parts[0].val * adjacent_parts[1].val;
                }
            });

        ratio_acc
    }
}

#[derive(Debug, Clone, Copy)]
struct Number {
    val: u32,
    origin: Coord,
    end: Coord,
}

impl Number {
    fn bounding_box(&self) -> (Coord, Coord) {
        (
            Coord { x: self.origin.x.saturating_sub(1) , y: self.origin.y.saturating_sub(1)},
            Coord { x: self.end.x.saturating_add(1) , y: self.end.y.saturating_add(1)},
        )
    }

    fn is_adjacent_to(&self, coord: &Coord) -> bool {
        let (min, max) = self.bounding_box();

        if coord.x < min.x || coord.x > max.x || 
            coord.y < min.y || coord.y > max.y {
                return false;
        }

        true
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum Symbol {
    NotPeriod,
    Cog,
}


#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));
    
    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "4361");

    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "467835");
}
