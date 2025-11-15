use crate::archetype;

pub struct Character {
    name: String,
    max_health: u32,
    current_health: u32,
    defense: u32,
    position: (i32, i32),
    chosen_archetype: archetype::Archetype,
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
            position: (0, 0),
        }
    }

    pub fn get_status(&self) -> String {
        return format!(
            "Name: {}\nHealth: {}/{}\nDefense: {}\nPosition: ({}, {})",
            self.name,
            self.current_health,
            self.max_health,
            self.defense,
            self.position.0,
            self.position.1
        );
    }

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

    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.max_health {
            self.max_health = 0;
        } else {
            self.max_health -= damage;
        }
    }

    pub fn heal(&mut self, amount: u32) {
        self.max_health += amount;
    }

    pub fn is_alive(&self) -> bool {
        self.max_health > 0
    }
}