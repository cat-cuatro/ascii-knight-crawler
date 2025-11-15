use std::io;
//use std::ptr::null;
mod character;
mod archetype;

fn main() {
    let mut character_name = String::new();
    println!("Name your character:");
    io::stdin().read_line(&mut character_name).expect("Failed to read line");
    let character_name = character_name.trim();
    println!("Welcome, {}!", character_name);
    let mut player_character = character::Character::new(character_name);
    println!("{:?}", player_character.get_status());
}
