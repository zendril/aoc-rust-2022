use std::str::FromStr;

use nom::{
    character::complete::{space0},
    IResult,
    sequence::separated_pair,
};
use nom::character::complete::{anychar, line_ending};
use nom::multi::separated_list1;

use crate::days::Day;

pub struct Day02;

#[derive(PartialEq, Eq, Debug)]
pub struct Attack(usize);

impl Attack {
    const ROCK: Self = Self(1);
    const PAPER: Self = Self(2);
    const SCISSORS: Self = Self(3);
}

impl FromStr for Attack {
    type Err = ();

    fn from_str(input: &str) -> Result<Attack, Self::Err> {
        match input {
            "X" | "A" => Ok(Attack::ROCK),
            "Y" | "B" => Ok(Attack::PAPER),
            "Z" | "C" => Ok(Attack::SCISSORS),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Outcome(usize);

impl Outcome {
    const WIN: Self = Self(6);
    const LOSE: Self = Self(0);
    const DRAW: Self = Self(3);
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Outcome::LOSE),
            "Y" => Ok(Outcome::DRAW),
            "Z" => Ok(Outcome::WIN),
            _ => Err(()),
        }
    }
}


fn parse_pair(input: &str) -> IResult<&str, (char, char)> {
    separated_pair(anychar, space0, anychar)(input)
}

pub struct Relationship {
    attack: Attack,
    win: Attack,
    lose: Attack,
    draw: Attack,
}

#[derive(Debug)]
pub struct MatchStat {
    _opponent_attack: Attack,
    my_attack: Attack,
    outcome: Outcome,
}

impl Day for Day02 {
    type Input = Vec<(char, char)>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let (remaining, lines) = separated_list1(line_ending, parse_pair)(input).unwrap();
        Ok((remaining, lines))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let rounds: Vec<(Attack, Attack)> = input.iter().map(|(opp, me)| { (Attack::from_str(&opp.to_string()).unwrap(), Attack::from_str(&me.to_string()).unwrap()) }).collect();

        let mut total_points: usize = 0;
        for (opponent_attack, my_attack) in rounds {
            // need to figure out how to compare r vs p vs s
            total_points += match my_attack {
                Attack::ROCK => {
                    if matches!(opponent_attack, Attack::PAPER) { my_attack.0 } else if matches!(opponent_attack, Attack::SCISSORS) { my_attack.0 + 6 } else { my_attack.0 + 3 }
                }
                Attack::PAPER => {
                    if matches!(opponent_attack, Attack::SCISSORS) { my_attack.0 } else if matches!(opponent_attack, Attack::ROCK) { my_attack.0 + 6 } else { my_attack.0 + 3 }
                }
                Attack::SCISSORS => {
                    if matches!(opponent_attack, Attack::ROCK) { my_attack.0 } else if matches!(opponent_attack, Attack::PAPER) { my_attack.0 + 6 } else { my_attack.0 + 3 }
                }
                _ => { 0 }
            }
        }
        total_points
    }

    type Output2 = usize;


    fn part_2(input: &Self::Input) -> Self::Output2 {

        let mut matches: Vec<MatchStat> = vec![];

        // a round
        for (opp_attack_char, outcome) in input {
            let opponent_attack = create_attack(Attack::from_str(&opp_attack_char.to_string()).unwrap());

            let outcome =  Outcome::from_str(&outcome.to_string()).unwrap();
            let match_stat = match outcome {
                Outcome::WIN => {
                    MatchStat {
                        _opponent_attack: opponent_attack.attack,
                        my_attack: create_attack(opponent_attack.lose).attack,
                        outcome,
                    }
                },
                Outcome::LOSE=> {
                    MatchStat {
                        _opponent_attack: opponent_attack.attack,
                        my_attack: create_attack(opponent_attack.win).attack,
                        outcome,
                    }
                },
                Outcome::DRAW => {
                    MatchStat {
                        _opponent_attack: opponent_attack.attack,
                        my_attack: create_attack(opponent_attack.draw).attack,
                        outcome,
                    }
                },
                _ => panic!()
            };
            matches.push(match_stat);
        }

        matches.iter().fold(0, |sum, x| sum + x.my_attack.0 + x.outcome.0)
    }
}

fn create_attack(attack: Attack) -> Relationship {
    match attack {
        Attack::ROCK => {
            Relationship {
                attack: Attack::ROCK,
                win: Attack::SCISSORS,
                lose: Attack::PAPER,
                draw: Attack::ROCK
            }
        },
        Attack::PAPER => {
            Relationship {
                attack: Attack::PAPER,
                win: Attack::ROCK,
                lose: Attack::SCISSORS,
                draw: Attack::PAPER,
            }
        },
        Attack::SCISSORS => {
            Relationship {
                attack: Attack::SCISSORS,
                win: Attack::PAPER,
                lose: Attack::ROCK,
                draw: Attack::SCISSORS,
            }
        }
        _ => panic!()
    }
}