use crate::util::input_read_to_string;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum RpsError {
    ParseError,
    OrderError,
    WhitespaceError,
}

impl From<Result<_, RpsError> for RspError {

}

type RpsResult<T> = std::result::Result<T, RpsError>;

impl core::fmt::Display for RpsError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            RpsError::ParseError => write!(f, "Failed to parse the input."),
            RpsError::OrderError => write!(f, "The input is misordered or malformed."),
            RpsError::WhitespaceError => write!(f, "The file's whitespace isn't as expected"),
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
enum RpsPlay {
    Rock,
    Paper,
    Scissors,
}

#[derive(Eq, PartialEq, Hash)]
enum RpsPlayer {
    Opp,
    You,
}

#[derive(Eq, PartialEq, Hash)]
struct Rps {
    play: RpsPlay,
    player: RpsPlayer,
}

static RPS: Lazy<HashMap<char, Rps>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        'A',
        Rps {
            play: RpsPlay::Rock,
            player: RpsPlayer::Opp,
        },
    );
    map.insert(
        'B',
        Rps {
            play: RpsPlay::Paper,
            player: RpsPlayer::Opp,
        },
    );
    map.insert(
        'C',
        Rps {
            play: RpsPlay::Scissors,
            player: RpsPlayer::Opp,
        },
    );
    map.insert(
        'X',
        Rps {
            play: RpsPlay::Rock,
            player: RpsPlayer::You,
        },
    );
    map.insert(
        'Y',
        Rps {
            play: RpsPlay::Paper,
            player: RpsPlayer::You,
        },
    );
    map.insert(
        'Z',
        Rps {
            play: RpsPlay::Scissors,
            player: RpsPlayer::You,
        },
    );
    map
});

fn rps(c: char, p: RpsPlayer) -> RpsResult<Rps> {
    let result = RPS.get(&c).ok_or(Err(RpsError::ParseError))?;
    if result.player != p {
        return Err(RpsError::ParseError);
    }
    Ok(*result)
}

fn rps_chars(iter: &mut std::str::Chars<'_>, player: RpsPlayer) -> RpsResult<Rps> {
    let character = iter.next().ok_or(Err(RpsError::OrderError))?;
    rps(character, player)
}

struct RpsScore {
    opp: u32,
    you: u32,
}

enum RpsOutcome {
    Win,
    Lose,
    Draw,
}

use RpsPlay::Paper;
use RpsPlay::Rock;
use RpsPlay::Scissors;

use RpsOutcome::Draw;
use RpsOutcome::Lose;
use RpsOutcome::Win;

fn play(opp: RpsPlay, you: RpsPlay) -> RpsOutcome {
    match opp {
        Rock => match you {
            Rock => Draw,
            Paper => Win,
            Scissors => Lose,
        },
        Paper => match you {
            Paper => Draw,
            Scissors => Win,
            Rock => Lose,
        },
        Scissors => match you {
            Scissors => Draw,
            Rock => Win,
            Paper => Lose,
        },
    }
}

fn score(opp: Rps, you: Rps, scoreboard: &mut RpsScore) {
    debug_assert!(opp.player == RpsPlayer::Opp);
    debug_assert!(you.player == RpsPlayer::You);
    let out = play(opp.play, you.play);
    match out {
        Win => scoreboard.you += 6,
        Lose => scoreboard.opp += 6,
        Draw => {
            scoreboard.you += 3;
            scoreboard.opp += 3;
        }
    };
}

fn score_match(line: &str, scoreboard: &mut RpsScore) -> RpsResult<()> {
    let mut iter = line.chars();
    let opp_move = rps_chars(&mut iter, RpsPlayer::Opp)?;
    let whitespace = iter.next();
    let _ = whitespace.ok_or_else(|| Err(RpsError::ParseError))?;
    if whitespace != ' ' {
        return Err(RpsError::WhitespaceError);
    }
    let your_move = rps_chars(&mut iter, RpsPlayer::You)?;
    if iter.next != None {
        return Err(RpsError::OrderError);
    }
    score(opp_move, your_move, scoreboard)
}

pub fn main() -> RpsResult<()> {
    let input = input_read_to_string("2");
    let mut output = RpsScore::new();
    let count = input.lines().count();
    for line in input.lines() {
        score_match(line, &mut output)?;
    }
    println!("Number of matches: {}", count);
    println!("Opponent's score: {}", output.opp);
    println!("Your Score: {}", output.you);
}
