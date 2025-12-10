pub struct MonsterArchetype {
    pub name: String,
    pub health: u32,
    pub attack: u32,
    pub defense: u32,
}

impl MonsterArchetype {
    pub fn new(name: &str, health: u32, attack: u32, defense: u32) -> Self {
        MonsterArchetype {
            name: name.to_string(),
            health,
            attack,
            defense,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_health(&self) -> u32 {
        self.health
    }
    pub fn get_attack(&self) -> u32 {
        self.attack
    }
    pub fn get_defense(&self) -> u32 {
        self.defense
    }
}
pub fn create_goblin() -> MonsterArchetype {
    MonsterArchetype::new("Goblin", 50, 10, 5)
}
pub fn create_slime() -> MonsterArchetype {
    MonsterArchetype::new("Slime", 30, 5, 2)
}
pub fn create_hobgoblin() -> MonsterArchetype {
    MonsterArchetype::new("Hobgoblin", 80, 15, 10)
}
pub fn create_imp() -> MonsterArchetype {
    MonsterArchetype::new("Imp", 40, 12, 3)
}
pub fn create_rat() -> MonsterArchetype {
    MonsterArchetype::new("Rat", 20, 4, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monster_archetype_creation() {
        let goblin = create_goblin();
        assert_eq!(goblin.get_name(), "Goblin");
    }

    #[test]
    fn test_attributes_of_monster_archetype() {
        let monster = MonsterArchetype::new("TestMonster", 100, 20, 10);
        assert_eq!(monster.get_health(), 100);
        assert_eq!(monster.get_attack(), 20);
        assert_eq!(monster.get_defense(), 10);
    }
}
