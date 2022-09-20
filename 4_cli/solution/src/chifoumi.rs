use rand::Rng;

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum Game {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&String> for Game {
    type Error = String;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "rock" => Ok(Game::Rock),
            "paper" => Ok(Game::Paper),
            "scissors" => Ok(Game::Scissors),
            _ => Err("Invalid game".to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GameResult {
    Win,
    Draw,
    Lost,
}

pub fn random_game() -> Game {
    let mut rng = rand::thread_rng();
    let val = rng.gen_range(0..3);
    match val {
        0 => Game::Rock,
        1 => Game::Paper,
        _ => Game::Scissors,
    }
}

pub fn play(a: Game, b: Game) -> GameResult {
    match (a, b) {
        // win
        (Game::Paper, Game::Rock) => GameResult::Win,
        (Game::Rock, Game::Scissors) => GameResult::Win,
        (Game::Scissors, Game::Paper) => GameResult::Win,
        // draw
        (Game::Rock, Game::Rock) => GameResult::Draw,
        (Game::Scissors, Game::Scissors) => GameResult::Draw,
        (Game::Paper, Game::Paper) => GameResult::Draw,
        // lose
        (Game::Rock, Game::Paper) => GameResult::Lost,
        (Game::Paper, Game::Scissors) => GameResult::Lost,
        (Game::Scissors, Game::Rock) => GameResult::Lost,
    }
}

#[cfg(test)]
mod tests {
    use super::play;
    use super::Game;
    use super::GameResult;

    #[test]
    fn test_player_one_wins() {
        assert_eq!(play(Game::Rock, Game::Scissors), GameResult::Win);
        assert_eq!(play(Game::Scissors, Game::Paper), GameResult::Win);
        assert_eq!(play(Game::Paper, Game::Rock), GameResult::Win);
    }

    #[test]
    fn test_player_one_draw() {
        assert_eq!(play(Game::Rock, Game::Rock), GameResult::Draw);
        assert_eq!(play(Game::Paper, Game::Paper), GameResult::Draw);
        assert_eq!(play(Game::Scissors, Game::Scissors), GameResult::Draw);
    }

    #[test]
    fn test_player_one_lose() {
        assert_eq!(play(Game::Rock, Game::Paper), GameResult::Lost);
        assert_eq!(play(Game::Paper, Game::Scissors), GameResult::Lost);
        assert_eq!(play(Game::Scissors, Game::Rock), GameResult::Lost);
    }
}
