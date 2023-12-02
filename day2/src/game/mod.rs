use std::str::FromStr;

mod parser;

pub(crate) struct Game {
    id: u64,
    sets: Vec<GameSet>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (remaining, game) = parser::parse_game(s).map_err(|err| err.to_string())?;

        if !remaining.is_empty() {
            return Err(format!("Unexpected remaining input: {remaining}"));
        }

        Ok(game)
    }
}

impl Game {
    pub(crate) fn id(&self) -> u64 {
        self.id
    }

    pub(crate) fn is_possible_with(
        &self,
        red_count: u64,
        green_count: u64,
        blue_count: u64,
    ) -> bool {
        self.sets.iter().all(|set| {
            set.red_count <= red_count
                && set.green_count <= green_count
                && set.blue_count <= blue_count
        })
    }

    pub(crate) fn minimal_set(&self) -> GameSet {
        self.sets
            .iter()
            .fold(GameSet::default(), |mut min_set, set| {
                if set.red_count > min_set.red_count {
                    min_set.red_count = set.red_count;
                }
                if set.green_count > min_set.green_count {
                    min_set.green_count = set.green_count;
                }
                if set.blue_count > min_set.blue_count {
                    min_set.blue_count = set.blue_count;
                }

                min_set
            })
    }
}

#[derive(Default)]
pub(crate) struct GameSet {
    red_count: u64,
    green_count: u64,
    blue_count: u64,
}

impl GameSet {
    pub(crate) fn power(&self) -> u64 {
        self.red_count * self.green_count * self.blue_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAME_1_SPEC: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    const GAME_3_SPEC: &str =
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

    #[test]
    fn test_game_possible_with() {
        let game1 = Game::from_str(GAME_1_SPEC).unwrap();
        let game3 = Game::from_str(GAME_3_SPEC).unwrap();

        assert!(game1.is_possible_with(12, 13, 14));
        assert!(!game3.is_possible_with(12, 13, 14));
    }

    #[test]
    fn test_game_minimal_set() {
        let game1 = Game::from_str(GAME_1_SPEC).unwrap();
        let game3 = Game::from_str(GAME_3_SPEC).unwrap();

        let min_set1 = game1.minimal_set();
        assert_eq!(4, min_set1.red_count);
        assert_eq!(2, min_set1.green_count);
        assert_eq!(6, min_set1.blue_count);

        let min_set3 = game3.minimal_set();
        assert_eq!(20, min_set3.red_count);
        assert_eq!(13, min_set3.green_count);
        assert_eq!(6, min_set3.blue_count);
    }
}
