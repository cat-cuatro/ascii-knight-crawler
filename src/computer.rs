use crate::character;
use rand::Rng;

pub struct Computer {
    //last_action: Option<u32>,
    //in_combat: bool,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            //last_action: None,
            //in_combat: false,
        }
    }

    #[allow(clippy::needless_late_init)] // when I tried to implement clippy's suggestion here, it broke the code
    pub fn random_agent(&self, character: &character::Character) -> &str {
        let mut rng = rand::rng();
        let dice;
        if character.peek_health().0 < character.peek_health().1 {
            dice = rng.random_range(1..=5);
        } else {
            dice = rng.random_range(1..=4);
        };
        match dice {
            1 => "n",
            2 => "s",
            3 => "e",
            4 => "w",
            5 => "h",
            _ => panic!("Invalid dice roll for computer agent"),
        }
    }

    // This would be a good place for me to add Q-Learning, and the project is set up to support it, but I don't
    // have enough time to implement it properly. So I'm keeping it as a random agent.
    #[allow(dead_code)]
    pub fn q_learning_agent(&self, _character: &character::Character) -> &str {
        "n" // placeholder
    }
}
