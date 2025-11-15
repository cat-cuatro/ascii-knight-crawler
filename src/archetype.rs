use std::io;
pub struct Archetype {
    pub name: String,
    pub health: u32,
    pub stamina: u32,
    pub defense: u32,
    pub mana: u32,
}

impl Archetype {
    pub fn new(name: &str, health: u32, stamina: u32, defense: u32, mana: u32) -> Self {
        Archetype {
            name: name.to_string(),
            health,
            stamina,
            defense,
            mana,
        }
    }
    pub fn clone(&self) -> Self {
        Archetype {
            name: self.name.clone(),
            health: self.health,
            stamina: self.stamina,
            defense: self.defense,
            mana: self.mana,
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
    pub fn get_mana(&self) -> u32 {
        self.mana
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub fn create_knight() -> Archetype {
    Archetype::new("Knight", 150, 100, 15, 10)
}

pub fn create_warrior() -> Archetype {
    Archetype::new("Warrior", 120, 150, 5, 0)
}


pub fn cycle_archetype_options() -> Archetype {
    println!("Choose your archetype:");
    println!("1. Knight");
    println!("2. Warrior");

    let mut archetype_choice = String::new();
    //let archetype_choice = archetype_choice.trim();
    loop {
        io::stdin().read_line(&mut archetype_choice).expect("Failed to read line");
        println!("You selected: {}", archetype_choice.trim());
        match archetype_choice.trim().as_ref() {
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