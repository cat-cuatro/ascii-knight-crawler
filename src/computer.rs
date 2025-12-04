use crate::character;
use crate::overworld;
use rand::Rng;


pub struct Computer {
    last_action: Option<u32>,
    in_combat: bool,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            last_action: None,
            in_combat: false,
        }
    }

    pub fn random_agent(&self, character: &character::Character) -> &str {
        let mut rng = rand::rng();
        let mut dice;
        if character.peek_health().0 < character.peek_health().1 {
            dice = rng.random_range(1..=5);
        }
        else {
            dice = rng.random_range(1..=4);
        }
        match dice {
            1 => "n",
            2 => "s",
            3 => "e",
            4 => "w",
            5 => "h",
            _ => "h",
        }
    }
}