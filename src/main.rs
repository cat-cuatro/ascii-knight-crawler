use std::io;
//use std::ptr::null;
use std::thread;
use std::time::Duration;
mod character;
mod archetype;
mod overworld;
mod world_tiles;
mod hostile_enemy;
mod monster_archetype;
mod computer;

fn main() {
    let mut character_name = String::new();
    println!("Name your character:");
    io::stdin().read_line(&mut character_name).expect("Failed to read line");
    let character_name = character_name.trim();
    println!("Welcome, {}!", character_name);
    let player_character = character::Character::new(character_name);
    println!("{:?}", player_character.get_status());
    //test_overworld(player_character);
    computer_play(player_character);
}

fn test_overworld(mut player_character: character::Character) {
    let mut overworld = overworld::Overworld::new((10, 10));
    overworld.new_event(player_character.get_position());
    overworld.update_character_position((-1, -1), player_character.get_position());
    loop {
        print!("\x1B[2J\x1B[H");
        overworld.print_overworld();
        println!("{}", player_character.get_status());
        player_character.observe_surroundings(&overworld);
        
        println!("\nEnter command (n/s/e/w to move, q to quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        
        let old_position = match input.trim() {
            "n" => player_character.move_north(),
            "s" => player_character.move_south(),
            "e" => {
                let mut target_position = player_character.get_position();
                target_position.0 += 1; // position to the east
                if player_character.check_for_enemy(&character::Direction::East) {
                    let temp = overworld.resolve_combat(&mut player_character, target_position);
                }
                player_character.move_east()
            },
            "w" => player_character.move_west(),
            "q" => break,
            _ => player_character.get_position(),
        };
        overworld.update_character_position(old_position, player_character.get_position());
        overworld.new_event(player_character.get_position());
    }
}

fn computer_play(mut computer_character: character::Character) {
    let mut computer = computer::Computer::new();
    let mut overworld = overworld::Overworld::new((10, 10));
    overworld.new_event(computer_character.get_position());
    overworld.update_character_position((-1, -1), computer_character.get_position());

    loop {
        print!("\x1B[2J\x1B[H");
        overworld.print_overworld();
        println!("{}", computer_character.get_status());
        computer_character.observe_surroundings(&overworld);
        let action = computer.random_agent(&computer_character);
        let old_position = match action {
            "n" => computer_character.move_north(),
            "s" => computer_character.move_south(),
            "e" => {
                let mut target_position = computer_character.get_position();
                target_position.0 += 1; // position to the east
                if computer_character.check_for_enemy(&character::Direction::East) {
                    let temp = overworld.resolve_combat(&mut computer_character, target_position);
                }
                computer_character.move_east()
            },
            "w" => computer_character.move_west(),
            _ => computer_character.get_position(),
        };
        overworld.update_character_position(old_position, computer_character.get_position());
        if computer_character.is_alive() == false {
            println!("Oh no! {} has been defeated!", computer_character.get_name());
            break;
        }
        thread::sleep(Duration::from_secs(5));
        overworld.new_event(computer_character.get_position());
    }
}
/*
fn test_overworld(mut player_character: character::Character) {
    let mut overworld = overworld::Overworld::new((10, 10));
    overworld.new_event(player_character.get_position());
    overworld.print_overworld();
    player_character.observe_surroundings(&overworld);
} */