use crate::archetype;
//use crate::world_tiles::Tile;
use crate::overworld;
use std::collections::HashMap;

pub struct Character {
    name: String,
    max_health: u32,
    current_health: u32,
    defense: u32,
    stamina: u32,
    coin: u32,
    position: (i32, i32),
    chosen_archetype: String,
    surrounding_tiles: HashMap<String, char>,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Northwest,
    Northeast,
    Southwest,
    Southeast,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::Northeast,
            Direction::Northwest,
            Direction::Southeast,
            Direction::Southwest,
        ]
    }

    pub fn to_string(&self) -> &str {
        match self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
            Direction::Northeast => "Northeast",
            Direction::Northwest => "Northwest",
            Direction::Southeast => "Southeast",
            Direction::Southwest => "Southwest",
        }
    }
}

impl Character {
    pub fn new(name: &str) -> Self {
        let class = archetype::cycle_archetype_options();
        Character {
            name: name.to_string(),
            chosen_archetype: class.get_name().to_string(),
            max_health: class.get_health(),
            current_health: class.get_health(),
            defense: class.get_defense(),
            stamina: class.get_stamina(),
            coin: 0,
            position: (4, 4),
            surrounding_tiles: HashMap::new(),
        }
    }

    pub fn new_custom(name: &str, archetype_name: &str, health: u32, stamina: u32, defense: u32) -> Self {
        let class = archetype::create_custom_archetype(archetype_name, health, stamina, defense);
        Character {
            name: name.to_string(),
            chosen_archetype: class.get_name().to_string(),
            max_health: class.get_health(),
            current_health: class.get_health(),
            defense: class.get_defense(),
            stamina: class.get_stamina(),
            coin: 0,
            position: (4, 4),
            surrounding_tiles: HashMap::new(),
        }
    }

    pub fn get_status(&self) -> String {
        format!(
            "Name: {} {} Health: {}/{} Defense: {} Coins: {} \nPosition: ({}, {}), Stamina: {}",
            self.chosen_archetype,
            self.name,
            self.current_health,
            self.max_health,
            self.defense,
            self.coin,
            self.position.0,
            self.position.1,
            self.stamina
        )
    }

    pub fn peek_health(&self) -> (u32, u32) {
        (self.current_health, self.max_health)
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.position.0, self.position.1)
    }

    pub fn observe_surroundings(&mut self, world: &overworld::Overworld) {
        self.surrounding_tiles.clear();
        for direction in Direction::all().iter() {
            let new_position = match direction {
                Direction::North => (self.position.0, self.position.1 - 1),
                Direction::South => (self.position.0, self.position.1 + 1),
                Direction::East => (self.position.0 + 1, self.position.1),
                Direction::West => (self.position.0 - 1, self.position.1),
                Direction::Northeast => (self.position.0 + 1, self.position.1 - 1),
                Direction::Northwest => (self.position.0 - 1, self.position.1 - 1),
                Direction::Southeast => (self.position.0 + 1, self.position.1 + 1),
                Direction::Southwest => (self.position.0 - 1, self.position.1 + 1),
            };
            if let Some(tile) = world.get_tile(new_position) {
                //println!("Tile at {:?}: {:?}", direction.to_string(), tile.get_symbol());
                self.surrounding_tiles
                    .insert(direction.to_string().to_string(), tile.get_symbol());
            } else {
                //println!("Tile at {:?}: Out of bounds", direction.to_string());
                self.surrounding_tiles
                    .insert(direction.to_string().to_string(), '#');
            }
        }
        //println!("Surrounding tiles: {:?}", self.surrounding_tiles);
    }

    pub fn check_for_enemy(&self, direction: &Direction) -> bool {
        if let Some(symbol) = self.surrounding_tiles.get(direction.to_string())
            && *symbol != '.'
            && *symbol != '#'
        {
            return true;
        }
        false
    }

    pub fn move_east(&mut self, walkable: bool) -> (i32, i32) {
        self.consume_stamina(1);
        let new_position = (self.position.0 + 1, self.position.1);
        let old_position = self.position;
        if walkable && self.surrounding_tiles.get("East") != Some(&'#') {
            self.position = new_position;
        }
        old_position
    }

    pub fn move_west(&mut self, walkable: bool) -> (i32, i32) {
        self.consume_stamina(1);
        let new_position = (self.position.0 - 1, self.position.1);
        let old_position = self.position;
        if walkable && self.surrounding_tiles.get("West") != Some(&'#') {
            self.position = new_position;
        }
        old_position
    }

    pub fn move_north(&mut self, walkable: bool) -> (i32, i32) {
        self.consume_stamina(1);
        let new_position = (self.position.0, self.position.1 - 1);
        let old_position = self.position;
        if walkable && self.surrounding_tiles.get("North") != Some(&'#') {
            self.position = new_position;
        }
        old_position
    }

    pub fn move_south(&mut self, walkable: bool) -> (i32, i32) {
        self.consume_stamina(1);
        let new_position = (self.position.0, self.position.1 + 1);
        let old_position = self.position;
        if walkable && self.surrounding_tiles.get("South") != Some(&'#') {
            self.position = new_position;
        }
        old_position
    }

    fn consume_stamina(&mut self, amount: u32) {
        self.stamina -= amount;
    }

    pub fn is_exhausted(&self) -> bool {
        self.stamina == 0
    }

    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.current_health {
            self.current_health = 0;
        } else {
            self.current_health -= damage;
        }
    }

    pub fn attack(&self) -> u32 {
        10
    }

    pub fn obtain_coin(&mut self, amount: u32) {
        self.coin += amount;
    }

    /*
    pub fn spend_coin(&mut self, amount: u32) -> bool {
        if self.coin >= amount {
            self.coin -= amount;
            true
        } else {
            false
        }
    }
    */
    pub fn heal(&mut self, amount: u32) {
        //if self.spend_coin(amount) {
        //    self.current_health += amount;
        //}
        self.current_health += amount;
    }

    pub fn is_alive(&self) -> bool {
        self.max_health > 0
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_creation() {
        let character = Character::new_custom("Hero", "CustomClass", 150, 100, 10);
        assert_eq!(character.name, "Hero");
        assert_eq!(character.chosen_archetype, "CustomClass");
        assert_eq!(character.max_health, 150);
        assert_eq!(character.current_health, 150);
        assert_eq!(character.defense, 10);
        assert_eq!(character.stamina, 100);
        assert_eq!(character.coin, 0);
        assert_eq!(character.position, (4, 4));
    }

    #[test]
    fn test_take_damage() {
        let mut character = Character::new_custom("Hero", "CustomClass", 100, 100, 10);
        character.take_damage(30);
        assert_eq!(character.current_health, 70);
        character.take_damage(80);
        assert_eq!(character.current_health, 0);
    }

    #[test]
    fn test_heal() {
        let mut character = Character::new_custom("Hero", "CustomClass", 100, 100, 10);
        character.take_damage(50);
        character.heal(30);
        assert_eq!(character.current_health, 80);
    }

    #[test]
    fn test_observe_surroundings() {
        let mut character = Character::new_custom("Hero", "CustomClass", 100, 100, 10);
        let overworld = overworld::Overworld::new((10, 10));
        character.observe_surroundings(&overworld);
        assert_eq!(character.surrounding_tiles.len(), 8);
    }

    #[test]
    fn test_movement() {
        let mut character = Character::new_custom("Hero", "CustomClass", 100, 100, 10);
        for direction in &[Direction::North, Direction::South, Direction::West, Direction::East] {
            let old_position = character.get_position();
            let new_old_position = match direction { // Test movement always returns old position
                Direction::North => character.move_north(true),
                Direction::East => character.move_east(true),
                Direction::West => character.move_west(true),
                Direction::South => character.move_south(true),
                _ => panic!("Invalid direction for movement test"),
            };
            assert_eq!(new_old_position, old_position);
            let expected_position = match direction { // Test new position is correct
                Direction::North => (old_position.0, old_position.1 - 1),
                Direction::East => (old_position.0 + 1, old_position.1),
                Direction::West => (old_position.0 - 1, old_position.1),
                Direction::South => (old_position.0, old_position.1 + 1),
                _ => panic!("Invalid direction for movement test"),
            };
            assert_eq!(character.get_position(), expected_position);
        }
    }
}