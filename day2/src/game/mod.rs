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
}

#[derive(Default)]
pub(crate) struct GameSet {
    red_count: u64,
    green_count: u64,
    blue_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_possible_with() {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let game1 = Game {
            id: 1,
            sets: vec![
                GameSet {
                    red_count: 3,
                    green_count: 0,
                    blue_count: 3,
                },
                GameSet {
                    red_count: 1,
                    green_count: 2,
                    blue_count: 6,
                },
                GameSet {
                    red_count: 0,
                    green_count: 2,
                    blue_count: 0,
                },
            ],
        };

        // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        let game3 = Game {
            id: 3,
            sets: vec![
                GameSet {
                    red_count: 20,
                    green_count: 8,
                    blue_count: 6,
                },
                GameSet {
                    red_count: 4,
                    green_count: 13,
                    blue_count: 5,
                },
                GameSet {
                    red_count: 1,
                    green_count: 5,
                    blue_count: 0,
                },
            ],
        };

        assert!(game1.is_possible_with(12, 13, 14));
        assert!(!game3.is_possible_with(12, 13, 14));
    }
}
