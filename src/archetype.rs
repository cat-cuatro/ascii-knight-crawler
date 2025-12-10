use std::io;
pub struct Archetype {
    pub name: String,
    pub health: u32,
    pub stamina: u32,
    pub defense: u32,
    //pub mana: u32,
}

impl Archetype {
    pub fn new(name: &str, health: u32, stamina: u32, defense: u32) -> Self {
        Archetype {
            name: name.to_string(),
            health,
            stamina,
            defense,
        }
    }
    pub fn get_health(&self) -> u32 {
        self.health
    }
    pub fn get_stamina(&self) -> u32 {
        self.stamina
    }
    pub fn get_defense(&self) -> u32 {
        self.defense
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub fn create_knight() -> Archetype {
    Archetype::new("Knight", 150, 200, 15)
}

pub fn create_warrior() -> Archetype {
    Archetype::new("Warrior", 120, 250, 5)
}

#[allow(dead_code)] // unused function for now
pub fn create_custom_archetype(name: &str, health: u32, stamina: u32, defense: u32) -> Archetype {
    Archetype::new(name, health, stamina, defense)
}

pub fn cycle_archetype_options() -> Archetype {
    println!("Choose your archetype:");
    println!("1. Knight");
    println!("2. Warrior");

    let mut archetype_choice = String::new();
    //let archetype_choice = archetype_choice.trim();
    loop {
        io::stdin()
            .read_line(&mut archetype_choice)
            .expect("Failed to read line");
        println!("You selected: {}", archetype_choice.trim());
        match archetype_choice.trim() {
            "1" => {
                println!("You have chosen the Knight archetype!");
                return create_knight();
            }
            "2" => {
                println!("You have chosen the Warrior archetype!");
                return create_warrior();
            }
            _ => {
                println!("Invalid choice, please select 1 or 2 or CTRL-C to exit.");
                archetype_choice.clear();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archetype_creation() {
        let knight = create_knight();
        assert_eq!(knight.name, "Knight");
        assert_eq!(knight.health, 150);
        assert_eq!(knight.stamina, 200);
        assert_eq!(knight.defense, 15);
    }
}
