struct Match {
    opponent: Hand,
    you: Hand,
}

impl Match {
    fn score(&self) -> u64 {
        let round_score = match (&self.you, &self.opponent) {
            // Loose
            (Hand::Rock, Hand::Paper) => 0,
            (Hand::Paper, Hand::Scissors) => 0,
            (Hand::Scissors, Hand::Rock) => 0,

            // Win
            (Hand::Rock, Hand::Scissors) => 6,
            (Hand::Paper, Hand::Rock) => 6,
            (Hand::Scissors, Hand::Paper) => 6,

            // Draw
            (_, _) => 3,
        };

        self.you.score() + round_score
    }

    // Part one
    fn try_from_one(value: &str) -> Result<Self, &'static str> {
        let mut hands = value.split(' ');

        let opponent = hands.next().ok_or("Missing opponent's hand")?.try_into()?;
        let you = hands.next().ok_or("Missing yourself's hand")?.try_into()?;

        Ok(Self { opponent, you })
    }

    // Part two
    fn try_from_two(value: &str) -> Result<Self, &'static str> {
        let mut hands = value.split(' ');

        let opponent: Hand = hands.next().ok_or("Missing opponent's hand")?.try_into()?;
        let you = opponent
            .to(hands.next().ok_or("Missing yourself's hand")?)
            .ok_or("Invalid yourself's hand")?;

        Ok(Self { opponent, you })
    }
}

#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(&self) -> u64 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn to(self, other: &str) -> Option<Hand> {
        match other {
            "X" => Some(self.to_lose()),
            "Y" => Some(self),
            "Z" => Some(self.to_win()),
            _ => None,
        }
    }

    fn to_lose(self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn to_win(self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

impl TryFrom<&str> for Hand {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err("Hand didn't match any"),
        }
    }
}

fn compute_score<F>(f: F) -> u64
where
    F: Fn(&str) -> Result<Match, &'static str>,
{
    std::fs::read_to_string("day_2/input")
        .expect("Input must exist")
        .split('\n')
        .map(|line| {
            if line.is_empty() {
                0
            } else {
                f(line).unwrap().score()
            }
        })
        .sum::<u64>()
}

fn main() {
    // Part one
    println!(
        "What would your total score: {}",
        compute_score(Match::try_from_one)
    );

    // Part two
    println!(
        "What would your total score according to original strategy: {}",
        compute_score(Match::try_from_two)
    );
}
