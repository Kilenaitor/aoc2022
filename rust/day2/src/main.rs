fn main() {
    part1();
    part2();
}

fn part2() {
    let score: i32 = include_str!("input.txt")
        .split("\n")
        .filter(|row| *row != "")
        .map(|round| {
            let (opponent, outcome) = parse_outcome_row(round);
            let my_move = get_my_move(&opponent, &outcome);
            let mut score = 0;
            match outcome {
                Outcome::WIN => score += 6,
                Outcome::DRAW => score += 3,
                Outcome::LOSS => score += 0,
            };
            match my_move {
                Move::ROCK => score += 1,
                Move::PAPER => score += 2,
                Move::SCISSORS => score += 3,
            }
            score
        })
        .sum();
    println!("Score: {}", score);
}

fn parse_outcome_row(row: &str) -> (Move, Outcome) {
    let (opponent, outcome) = row.split_once(" ").unwrap();
    (parse_opponent_move(opponent), parse_outcome(outcome))
}

fn parse_outcome(input: &str) -> Outcome {
    match input {
        "X" => Outcome::LOSS,
        "Y" => Outcome::DRAW,
        "Z" => Outcome::WIN,
        _ => panic!("Invalid outcome"),
    }
}

fn get_my_move(opponent: &Move, outcome: &Outcome) -> Move {
    match opponent {
        Move::ROCK => match outcome {
            Outcome::LOSS => Move::SCISSORS,
            Outcome::DRAW => Move::ROCK,
            Outcome::WIN => Move::PAPER,
        },
        Move::PAPER => match outcome {
            Outcome::LOSS => Move::ROCK,
            Outcome::DRAW => Move::PAPER,
            Outcome::WIN => Move::SCISSORS,
        },
        Move::SCISSORS => match outcome {
            Outcome::LOSS => Move::PAPER,
            Outcome::DRAW => Move::SCISSORS,
            Outcome::WIN => Move::ROCK,
        },
    }
}

fn part1() {
    let score: i32 = include_str!("input.txt")
        .split("\n")
        .filter(|row| *row != "")
        .map(|round| {
            let (opponent, me) = parse_play_row(round);
            let outcome = get_outcome(&opponent, &me);
            let mut score = 0;
            match outcome {
                Outcome::WIN => score += 6,
                Outcome::DRAW => score += 3,
                Outcome::LOSS => score += 0,
            };
            match me {
                Move::ROCK => score += 1,
                Move::PAPER => score += 2,
                Move::SCISSORS => score += 3,
            }
            score
        })
        .sum();
    println!("Score: {}", score);
}

fn parse_play_row(row: &str) -> (Move, Move) {
    let (opponent, me) = row.split_once(" ").unwrap();
    (parse_opponent_move(opponent), parse_my_move(me))
}

fn parse_opponent_move(input: &str) -> Move {
    match input {
        "A" => Move::ROCK,
        "B" => Move::PAPER,
        "C" => Move::SCISSORS,
        _ => panic!("Invalid opponent move"),
    }
}

fn parse_my_move(input: &str) -> Move {
    match input {
        "X" => Move::ROCK,
        "Y" => Move::PAPER,
        "Z" => Move::SCISSORS,
        _ => panic!("Invalid opponent move"),
    }
}

fn get_outcome(opponent_move: &Move, my_move: &Move) -> Outcome {
    match opponent_move {
        Move::ROCK => match my_move {
            Move::ROCK => Outcome::DRAW,
            Move::PAPER => Outcome::WIN,
            Move::SCISSORS => Outcome::LOSS,
        },
        Move::PAPER => match my_move {
            Move::ROCK => Outcome::LOSS,
            Move::PAPER => Outcome::DRAW,
            Move::SCISSORS => Outcome::WIN,
        },
        Move::SCISSORS => match my_move {
            Move::ROCK => Outcome::WIN,
            Move::PAPER => Outcome::LOSS,
            Move::SCISSORS => Outcome::DRAW,
        },
    }
}

enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}
enum Outcome {
    WIN,
    LOSS,
    DRAW,
}
