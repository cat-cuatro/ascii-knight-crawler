use crate::monster_archetype;
use rand::Rng; 

pub struct HostileEnemy {
    name: String,
    max_health: u32,
    current_health: u32,
    attack: u32,
    defense: u32,
    position: (i32, i32),
    monster_archetype: monster_archetype::MonsterArchetype,
}

impl HostileEnemy {
    pub fn new(position: (i32, i32), difficulty: u32) -> Self {
        let mut rng = rand::rng();
        let mut dice = rng.random_range(1..=2);
        if difficulty >= 3 {
            dice = rng.random_range(1..=5);
        }
        let monster = match dice {
            1 => monster_archetype::create_goblin(),
            2 => monster_archetype::create_slime(),
            3 => monster_archetype::create_imp(),
            4 => monster_archetype::create_rat(),
            5 => monster_archetype::create_hobgoblin(),
            _ => panic!("Invalid dice roll for monster archetype"),
        };
        HostileEnemy {
            name: monster.get_name().to_string(),
            max_health: monster.get_health(),
            current_health: monster.get_health(),
            attack: monster.get_attack(),
            defense: monster.get_defense(),
            position: position,
            monster_archetype: monster,
        }
    }
    // I'm putting movement here, but not using it for the initial implementation to keep scope manageable
    /*
    pub fn move_east(&mut self) {
        let new_position = (self.position.0 + 1, self.position.1);
        self.position = new_position;
    }

    pub fn move_west(&mut self) {
        let new_position = (self.position.0 - 1, self.position.1);
        self.position = new_position;
    }

    pub fn move_north(&mut self) {
        let new_position = (self.position.0, self.position.1 - 1);
        self.position = new_position;
    }

    pub fn move_south(&mut self) {
        let new_position = (self.position.0, self.position.1 + 1);
        self.position = new_position;
    }
    */
    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.max_health {
            self.max_health = 0;
        } else {
            self.max_health -= damage;
        }
    }

    pub fn drop_loot(&self) -> u32 {
        let mut rng = rand::rng();
        let loot_amount = rng.random_range(5..=15);
        loot_amount
    }

    pub fn attack(&self) -> u32 {
        self.attack
    }

    pub fn heal(&mut self, amount: u32) {
        self.max_health += amount;
    }

    pub fn is_alive(&self) -> bool {
        self.max_health > 0
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}