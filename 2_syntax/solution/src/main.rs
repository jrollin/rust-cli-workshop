fn main() {
    println!("Hello, i am Crabby 🦀 !");
}

fn add(a: u8, b: u8) -> u8 {
    a + b
}

#[derive(Debug)]
pub enum Game {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
pub enum GameResult {
    Win,
    Draw,
    Lost,
}

fn play(a: Game, b: Game) -> GameResult {
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
    use super::add;
    use super::play;
    use super::Game;
    use super::GameResult;

    #[test]
    fn test_add() {
        assert_eq!(add(12, 5), 17);
    }

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
