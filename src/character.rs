use crate::archetype;
//use crate::world_tiles::Tile;
use crate::overworld;
use std::collections::HashMap;

pub struct Character {
    name: String,
    max_health: u32,
    current_health: u32,
    defense: u32,
    coin: u32,
    position: (i32, i32),
    chosen_archetype: archetype::Archetype,
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
            chosen_archetype: class.clone(),
            max_health: class.health,
            current_health: class.health,
            defense: class.defense,
            coin: 0,
            position: (4, 4),
            surrounding_tiles: HashMap::new(),
        }
    }

    pub fn get_status(&self) -> String {
        return format!(
            "Name: {} Health: {}/{} Defense: {} Coins: {} Position: ({}, {})",
            self.name,
            self.current_health,
            self.max_health,
            self.defense,
            self.coin,
            self.position.0,
            self.position.1
        );
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
                self.surrounding_tiles.insert(direction.to_string().to_string(), tile.get_symbol());
            }
        }
        //println!("Surrounding tiles: {:?}", self.surrounding_tiles);
    }

    pub fn check_for_enemy(&self, direction: &Direction) -> bool {
        if let Some(symbol) = self.surrounding_tiles.get(&direction.to_string().to_string()) {
            if *symbol == 'G' || *symbol == 'S' {
                return true;
            }
        }
        return false;
    }

    pub fn move_east(&mut self) -> (i32, i32) {
        let new_position = (self.position.0 + 1, self.position.1);
        let old_position = self.position;
        if self.check_for_enemy(&Direction::East) == false {
            self.position = new_position;
        }
        return old_position;
    }

    pub fn move_west(&mut self) -> (i32, i32) {
        let new_position = (self.position.0 - 1, self.position.1);
        let old_position = self.position;
        self.position = new_position;
        return old_position;
    }

    pub fn move_north(&mut self) -> (i32, i32) {
        let new_position = (self.position.0, self.position.1 - 1);
        let old_position = self.position;
        self.position = new_position;
        return old_position;
    }

    pub fn move_south(&mut self) -> (i32, i32) {
        let new_position = (self.position.0, self.position.1 + 1);
        let old_position = self.position;
        self.position = new_position;
        return old_position;
    }

    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.max_health {
            self.max_health = 0;
        } else {
            self.max_health -= damage;
        }
    }

    pub fn detect_and_attack_surrounding_enemies(&self) -> bool {
        for symbol in self.surrounding_tiles.values() {
            if *symbol == 'G' || *symbol == 'S' {
                return true;
            }
        }
        return false;
    }

    pub fn attack(&self) -> u32 {
        10
    }

    pub fn obtain_coin(&mut self, amount: u32) {
        self.coin += amount;
    }

    pub fn spend_coin(&mut self, amount: u32) -> bool {
        if self.coin >= amount {
            self.coin -= amount;
            true
        } else {
            false
        }
    }

    pub fn heal(&mut self, amount: u32) {
        if self.spend_coin(amount) {
            self.current_health += amount;
        }
        self.current_health += amount;
    }

    pub fn is_alive(&self) -> bool {
        self.max_health > 0
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}