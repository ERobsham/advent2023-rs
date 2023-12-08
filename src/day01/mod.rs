use crate::Part;

enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    fn parse_from(value: &str, only_digits: bool) -> Option<Self> {   
        match value {
            l if l.starts_with("1") => Some(Self::One),
            l if l.starts_with("2") => Some(Self::Two),
            l if l.starts_with("3") => Some(Self::Three),
            l if l.starts_with("4") => Some(Self::Four),
            l if l.starts_with("5") => Some(Self::Five),
            l if l.starts_with("6") => Some(Self::Six),
            l if l.starts_with("7") => Some(Self::Seven),
            l if l.starts_with("8") => Some(Self::Eight),
            l if l.starts_with("9") => Some(Self::Nine),
            
            l if !only_digits && l.starts_with("one") => Some(Self::One),
            l if !only_digits && l.starts_with("two") => Some(Self::Two),
            l if !only_digits && l.starts_with("three") => Some(Self::Three),
            l if !only_digits && l.starts_with("four") => Some(Self::Four),
            l if !only_digits && l.starts_with("five") => Some(Self::Five),
            l if !only_digits && l.starts_with("six") => Some(Self::Six),
            l if !only_digits && l.starts_with("seven") => Some(Self::Seven),
            l if !only_digits && l.starts_with("eight") => Some(Self::Eight),
            l if !only_digits && l.starts_with("nine") => Some(Self::Nine),

            _ => None,
        }
    }
}

impl Into<u32> for Digit {
    fn into(self) -> u32 {
        match self {
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }
}

pub(crate) fn solve(input: Box<dyn Iterator<Item = String>>, part: Part) -> String {

    let total: u32 = input
        .map(|line| {
            let l = line.as_str();
            let nums: Vec<u32> = (0..l.len()).filter_map(|idx| {
                parse_digit(&l[idx..l.len()], part) 
            }).collect();

            (nums.first().expect("must have at least one digit per line") * 10) +
            nums.last().expect("must have at least one digit per line")
        }).sum();

    format!("{}", total)
}

fn parse_digit(line: &str, part: Part) -> Option<u32> {
    let d = match part {
        Part::Part1 => Digit::parse_from(line, true),
        Part::Part2 => Digit::parse_from(line, false),
    };
    match d {
        Some(d) => Some(d.into()),
        None => None,
    }
}


#[test]
// sanity check vs example input
fn test_input_part1() {
    const EXAMPLE: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    let output = solve(Box::new(lines), Part::Part1);
    assert_eq!(output.as_str(), "142");
}

#[test]
fn test_input_part2() {
    const EXAMPLE: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));
    
    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "281");
}
