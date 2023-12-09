use std::collections::HashMap;

use crate::Part;

use nom::{
    bytes::complete::tag,
    character::complete::{self, space1},
    error::VerboseError,
    sequence::{preceded, terminated, tuple, separated_pair},
    multi::separated_list0,
    IResult,
};

pub(crate) fn solve(input: Box<dyn Iterator<Item = String>>, part: Part) -> String {

    let cards: Vec<Card> = input.filter_map(|line|  {
            let (_, card) = parse_card(line.as_str()).ok()?;
            Some(card)
        }).collect();

    let total = match part {
        Part::Part1 => {
            cards.iter().map(|c| c.value()).sum()
        }
        Part::Part2 => {
            let mut card_counts: HashMap<usize, u32> = HashMap::new();
            let total_cards = cards.len();
            cards.iter().enumerate().for_each(|(idx, c)|{
                let win_count = c.num_winners();
                let nxt = idx+1;

                card_counts.entry(idx)
                    .and_modify(|v| *v = *v+1 )
                    .or_insert(1);

                let cur_count = *card_counts.get(&idx).expect("already added");                
                (nxt..(nxt+win_count)).into_iter()
                    .for_each(|i| {
                        if i > total_cards { return; }

                        card_counts.entry(i)
                            .and_modify(|v| *v = *v+cur_count )
                            .or_insert(cur_count);
                    })
            });

            card_counts.values().sum::<u32>()
        }
    };
        

    format!("{}", total)
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    revealed_numbers: Vec<u32>,
}

impl Card {
    fn value(&self) -> u32 {
        let num_winners = self.num_winners() as u32;

        if num_winners > 0 {
            2u32.pow(num_winners - 1)
        } else {
            0
        }
    }

    fn num_winners(&self) -> usize {
        self.revealed_numbers.iter().filter_map(|n| {
            if self.winning_numbers.contains(n) {
                Some(n)
            } else { 
                None 
            }
        }).count()
    }

}

fn parse_card(input: &str) -> IResult<&str, Card, VerboseError<&str>> {
    let (input, id) = preceded(
            tuple((tag("Card"), space1)),
            terminated(complete::u32, tuple((tag(":"),space1))),
        )(input)?;
    let (input, (winning_numbers, revealed_numbers)) = separated_pair(
            separated_list0(space1, complete::u32),
            separated_pair(space1, tag("|"), space1),
            separated_list0(space1, complete::u32),
        )(input)?;

    Ok((
        input,
        Card{
            id,
            winning_numbers,
            revealed_numbers,
        },
    ))
}

#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));
    
    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "13");

    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "30");
}
