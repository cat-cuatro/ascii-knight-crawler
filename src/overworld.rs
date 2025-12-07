use crate::world_tiles::Tile;
use crate::character;
use rand::Rng;

pub struct Overworld {
    tiles: Vec<Vec<Tile>>,
    survival_score: u32,
    ticks: u32,
    difficulty: u32,
}

impl Overworld {
    pub fn new(grid_size: (i32, i32)) -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for y in 0..grid_size.1 {
            let mut row: Vec<Tile> = Vec::new();
            for x in 0..grid_size.0 {
                let tile = Tile::new('.', true, (x, y));
                row.push(tile);
            }
            tiles.push(row);
        }
        Overworld {
            tiles, 
            survival_score: 0,
            ticks: 0,
            difficulty: 1,
        }
    }

    pub fn new_event(&mut self, character_location: (i32, i32)) {
        let mut rng = rand::rng();
        let _event_rate = (10/self.difficulty) as i32;
        if self.ticks % _event_rate as u32 != 0 {
            return;
        }
        let spawn_coords = loop {
            let x = rng.random_range(0..self.tiles[0].len() as i32);
            let y = rng.random_range(0..self.tiles.len() as i32);
            if (x, y) != character_location {
                self.tiles[y as usize][x as usize].spawn_enemy(self.difficulty);
                println!("Spawned enemy at ({}, {})", x, y);
                break (x, y);
            }
        };
        //let x = rng.random_range(0..self.tiles[0].len() as i32);
        //let y = rng.random_range(0..self.tiles.len() as i32);
        //self.tiles[spawn_coords.1 as usize][spawn_coords.0 as usize].spawn_enemy();
    }

    pub fn update_character_position(&mut self, old_position: (i32, i32), new_position: (i32, i32)) {
        let mut old_tile = self.get_tile_mut(old_position);
        if let Some(tile) = old_tile.as_mut() {
            tile.unset_character();
        }
        let mut new_tile = self.get_tile_mut(new_position);
        if let Some(tile) = new_tile.as_mut() {
            tile.set_character();
        }   
    }
    
    pub fn print_overworld(&self) {
        println!("Overworld:");
        println!("Survival Score: {}, Difficulty: {}, Ticks: {}", self.survival_score, self.difficulty, self.ticks);
        for row in &self.tiles {
            for tile in row {
                print!(" {} ", tile.get_symbol());
            }
            println!();
        }
    }

    pub fn get_tile(&self, position: (i32, i32)) -> Option<&Tile> {
        if position.0 < 0 || position.1 < 0 || position.1 as usize >= self.tiles.len() || position.0 as usize >= self.tiles[0].len() {
            //return &Tile::new('#', false, position);
            return None
        }
        return Some(&self.tiles[position.1 as usize][position.0 as usize])
    }

    fn get_tile_mut(&mut self, position: (i32, i32)) -> Option<&mut Tile> {
        if position.0 < 0 || position.1 < 0 || position.1 as usize >= self.tiles.len() || position.0 as usize >= self.tiles[0].len() {
            return None
        }
        Some(&mut self.tiles[position.1 as usize][position.0 as usize])
    }

    pub fn player_command_attack(&mut self, target_position: (i32, i32), mut player_character: character::Character) -> bool {
        let tile = self.get_tile_mut(target_position);
        if tile.is_some() {
            if let Some(enemy) = tile.expect("None or an enemy").get_enemy_mut() {
                enemy.take_damage(player_character.attack());
                println!("Attacked enemy at position {:?} for {} damage!", target_position, player_character.attack());
                player_character.take_damage(enemy.attack());
                return true;
            }
        }
        return false;
    }

    pub fn resolve_combat(&mut self, character: &mut character::Character, target_position: (i32, i32)) -> bool{
        let mut tile_walkable = true;
        if let Some(tile) = self.get_tile_mut(target_position) {
            if let Some(enemy) = tile.get_enemy_mut() {
                let char_damage = character.attack();
                enemy.take_damage(char_damage);
                if enemy.is_alive() {
                    let enemy_damage = enemy.attack();
                    character.take_damage(enemy_damage);
                    println!("Combat: Character dealt {} damage, took {} damage", char_damage, enemy_damage);
                    tile_walkable = false;
                } else {
                    character.obtain_coin(enemy.drop_loot());
                    tile.despawn_enemy();
                    println!("Enemy defeated!");
                    self.survival_score += 1;
                    tile_walkable = true;
                }
            }
        }
        return tile_walkable;
    }

    pub fn tick_world(&mut self) {
        self.ticks += 1;
        if self.ticks % 20 == 0 {
            self.advance_difficulty();
            println!("World difficulty increased to {}", self.difficulty);
        }
    }

    fn advance_difficulty(&mut self) {
        if self.difficulty >= 10 {
            return;
        }
        self.difficulty += 1;
    }
}