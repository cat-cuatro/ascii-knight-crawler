use crate::character;
use crate::overworld;


pub fn act(computer_character: &mut character::Character, world: &overworld::Overworld) {
    // Placeholder for future AI actions
}

pub fn move_randomly(computer_character: &mut character::Character) {
    let mut rng = rand::rng();
    let dice = rng.random_range(1..=4);
    match dice {
        1 => computer_character.move_north(),
        2 => computer_character.move_south(),
        3 => computer_character.move_east(),
        4 => computer_character.move_west(),
        _ => {},
    }
}

pub fn enemy_is_nearby(computer_character: &character::Character, world: &overworld::Overworld) -> bool {
    for symbol in computer_character.surrounding_tiles.values() {
        if *symbol == 'G' || *symbol == 'S' {
            return true;
        }
    }
    return false;
}