static INPUT: &'static str = include_str!("../data/input");

fn main() {
    println!("Part 1: {}", INPUT
        .split("\n")
        .filter(|line| line.len() == 3)
        .map(|el| {
            let mut chars = el.chars();
            let opponent = to_hand(&chars.nth(0).unwrap());
            let me = to_hand(&chars.nth(1).unwrap());
            result_p1(&opponent, &me)
        }).sum::<u64>());

    println!("Part 2: {}", INPUT
        .split("\n")
        .filter(|line| line.len() == 3)
        .map(|el| {
            let mut chars = el.chars();
            let opponent = to_hand(&chars.nth(0).unwrap());
            let me = to_strategy(&chars.nth(1).unwrap());
            result_p2(&opponent, &me)
        }).sum::<u64>());
}

fn result_p1(opponent: &Hand, me: &Hand) -> u64 {
    (match (opponent, me) {
        (Hand::Rock, Hand::Scissors) | (Hand::Paper, Hand::Rock) | (Hand::Scissors, Hand::Paper) => 0,
        (Hand::Scissors, Hand::Rock) | (Hand::Rock, Hand::Paper) | (Hand::Paper, Hand::Scissors) => 6,
        _ => 3
    })+&me.to_u64()
}

fn result_p2(opponent: &Hand, strategy: &Strategy) -> u64 {
    (match (opponent, strategy) {
        (Hand::Rock, Strategy::Lose)     | (Hand::Paper, Strategy::Win)    | (Hand::Scissors, Strategy::Draw) => Hand::Scissors,
        (Hand::Paper, Strategy::Lose)    | (Hand::Scissors, Strategy::Win) | (Hand::Rock, Strategy::Draw)     => Hand::Rock,
        (Hand::Scissors, Strategy::Lose) | (Hand::Rock, Strategy::Win)     | (Hand::Paper, Strategy::Draw)    => Hand::Paper,
        _ => Hand::Invalid
    }).to_u64()+strategy.to_u64()
}

enum Hand {
    Rock,
    Paper,
    Scissors,
    Invalid
}

enum Strategy {
    Lose,
    Draw,
    Win,
    Invalid
}

impl Hand {
    fn to_u64(&self) -> u64 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
            Hand::Invalid => 0
        }
    }
}

impl Strategy {
    fn to_u64(&self) -> u64 {
        match self {
            Strategy::Lose => 0,
            Strategy::Draw => 3,
            Strategy::Win => 6,
            Strategy::Invalid => 0
        }
    }
}

fn to_hand(value: &char) -> Hand {
    match value {
        'A' | 'X' => Hand::Rock,
        'B' | 'Y' => Hand::Paper,
        'C' | 'Z' => Hand::Scissors,
        _ => Hand::Invalid
    }
}

fn to_strategy(value: &char) -> Strategy {
    match value {
        'X' => Strategy::Lose,
        'Y' => Strategy::Draw,
        'Z' => Strategy::Win,
        _ => Strategy::Invalid
    }
}


