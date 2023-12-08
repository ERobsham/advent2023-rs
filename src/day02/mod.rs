use crate::Part;

use std::collections::HashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{self, space1, space0},
    error::{self, VerboseError, ParseError},
    sequence::{preceded, terminated, tuple, separated_pair},
    multi::separated_list0,
    IResult,
};

pub(crate) fn solve(input: Box<dyn Iterator<Item = String>>, part: Part) -> String {

    let game_constraints = Round {
        cubes_played: vec![ 
            (CubeType::Red, 12),
            (CubeType::Green, 13),
            (CubeType::Blue, 14),
        ].into_iter().collect(),
    };

    let games:Vec<Game> = input.filter_map(|line|  {
            let (_, game_obj) = game(line.as_str()).ok()?;
            Some(game_obj)
        }).collect();

    let total: u32;
    match part {
        Part::Part1 => {
            total = games.iter().fold(0u32, |acc, g| {
                if g.is_valid_for_constraints(&game_constraints) {
                    acc + g.id
                } else {
                    acc
                }
            });
        }
        Part::Part2 => {
            total = games.iter().fold(0u32, |acc, g| {
                acc + g.min_constraints().power()
            })
        }
    }
        

    format!("{}", total)
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_valid_for_constraints(&self, round_constraints: &Round) -> bool {
        self.rounds.iter()
            .fold(true, |acc, cur| acc && cur.is_valid_for_constraints(round_constraints))
    }

    fn min_constraints(&self) -> Round {
        let min_constraints = self.rounds.iter()
            .map(|round| round.min_constraints())
            .fold( Round::empty_round(), |mut acc, cur_round| {
                cur_round.update_constraints(&mut acc);
                acc
            });
        min_constraints
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    cubes_played: HashMap<CubeType, u32>,
}

impl Round {
    fn empty_round() -> Round {
        let no_cubes_played = vec![
                (CubeType::Red, 0u32),
                (CubeType::Green, 0u32),
                (CubeType::Blue, 0u32),
            ].into_iter().collect();
        Round { cubes_played: no_cubes_played }
    }

    fn is_valid_for_constraints(&self, round_constraints: &Round) -> bool {
        self.cubes_played.iter()
            .fold(true, |acc, (k, v)| acc && {
                let max_cubes = round_constraints.cubes_played.get(k).expect("constraints must have all cube types");
                max_cubes >= v
            })
    }

    fn min_constraints(&self) -> Round {
        Round { cubes_played: vec![
            (CubeType::Red, *self.cubes_played.get(&CubeType::Red).unwrap_or(&0)),
            (CubeType::Green, *self.cubes_played.get(&CubeType::Green).unwrap_or(&0)),
            (CubeType::Blue, *self.cubes_played.get(&CubeType::Blue).unwrap_or(&0)),
        ].into_iter().collect() }
    }

    fn update_constraints(&self, round_constraints: &mut Round) {
        let r_num = *self.cubes_played.get(&CubeType::Red).unwrap_or(&0);
        let g_num = *self.cubes_played.get(&CubeType::Green).unwrap_or(&0);
        let b_num = *self.cubes_played.get(&CubeType::Blue).unwrap_or(&0);

        round_constraints.cubes_played.entry(CubeType::Red)
            .and_modify(|v| *v = u32::max(r_num, *v) );
        round_constraints.cubes_played.entry(CubeType::Green)
            .and_modify(|v| *v = u32::max(g_num, *v) );
        round_constraints.cubes_played.entry(CubeType::Blue)
            .and_modify(|v| *v = u32::max(b_num, *v) );
    }

    fn power(&self) -> u32 {
        let r_num = *self.cubes_played.get(&CubeType::Red).unwrap_or(&1);
        let g_num = *self.cubes_played.get(&CubeType::Green).unwrap_or(&1);
        let b_num = *self.cubes_played.get(&CubeType::Blue).unwrap_or(&1);

        r_num * g_num * b_num
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum CubeType {
    Red,
    Green,
    Blue,
}

// `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`
fn game(input: &str) -> IResult<&str, Game, VerboseError<&str>> {
    let (input, id) = terminated(
        preceded( tuple((tag("Game"), space1)), complete::u32 ),
        tuple((tag(":"), space1)),
    )(input)?;

    let (input, rounds) = separated_list0(
            tuple( (tag(";"), space1) ),
            round,
        )(input)?;

    Ok((
        input,
        Game {
            id,
            rounds,
        },
    ))
}

// '1 red, 2 green, 6 blue'
fn round(input: &str) -> IResult<&str, Round, VerboseError<&str>> {
    let (input, cubes_list) = separated_list0(
        tuple((tag(","), space0)),
        cube,
    )(input)?;

    let mut cubes_played: HashMap<CubeType, u32> = HashMap::new();
    cubes_list.iter().for_each(|(cube_type, num)| {
        cubes_played.entry(*cube_type)
            .and_modify(|v| *v += num )
            .or_insert(*num);
    });

    Ok((
        input,
        Round{
            cubes_played,
        }
    ))
}

// '1 red' || '6 blue' etc
fn cube(input: &str) -> IResult<&str, (CubeType, u32), VerboseError<&str>> {
    let (input, (num, str_type)) = separated_pair(complete::u32, space1, complete::alpha1)(input)?;
    let cube_type = match str_type {
        "red" => CubeType::Red,
        "green" => CubeType::Green,
        "blue" => CubeType::Blue,
        _ => return Err(nom::Err::Error(VerboseError::from_error_kind(input, error::ErrorKind::IsNot))),
    };

    Ok((
        input,
        (cube_type, num)
    ))
}


#[test]
fn test_parser() {
    const EXAMPLE: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    let (remaining, parsed) = game(EXAMPLE).expect("valid example input");
    
    assert_eq!(remaining, "");
    assert_eq!(parsed, 
        Game{
            id:1, 
            rounds: vec![ 
                Round{cubes_played:HashMap::from([(CubeType::Blue, 3), (CubeType::Red, 4)])},
                Round{cubes_played:HashMap::from([(CubeType::Red, 1), (CubeType::Green, 2), (CubeType::Blue, 6)])},
                Round{cubes_played:HashMap::from([(CubeType::Green, 2)])},
                ],
        });
}

#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));
    
    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "8");

    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "2286");
}
