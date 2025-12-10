use crate::hostile_enemy::HostileEnemy;
//use rand::Rng;

pub struct Tile {
    symbol: char,
    walkable: bool,
    //description: String,
    //item: Option<String>,
    pub npc: Option<HostileEnemy>,
    position: (i32, i32),
}

impl Tile {
    pub fn new(symbol: char, walkable: bool, position: (i32, i32)) -> Self {
        Tile {
            symbol,
            walkable,
            //description: description.to_string(),
            //item: None,
            npc: None,
            position,
        }
    }

    pub fn spawn_enemy(&mut self, difficulty: u32) {
        let enemy = HostileEnemy::new((self.position.0, self.position.1), difficulty);
        self.npc = Some(enemy);
        self.symbol = match self.npc.as_ref().unwrap().get_name() {
            "Goblin" => 'G',
            "Slime" => 'S',
            "Hobgoblin" => 'H',
            "Imp" => 'I',
            "Rat" => 'R',
            _ => panic!("Unknown enemy type"),
        };
        self.walkable = false;
    }

    pub fn set_character(&mut self) {
        self.symbol = '@';
        self.walkable = false;
    }

    pub fn unset_character(&mut self) {
        self.symbol = '.';
        self.walkable = true;
    }

    //pub fn get_enemy(&self) -> Option<&HostileEnemy> {
    //    self.npc.as_ref()
    //}

    pub fn get_enemy_mut(&mut self) -> Option<&mut HostileEnemy> {
        self.npc.as_mut()
    }

    pub fn despawn_enemy(&mut self) {
        self.npc = None;
        self.symbol = '.';
        self.walkable = true;
    }

    pub fn get_symbol(&self) -> char {
        self.symbol
    }

    pub fn is_walkable(&self) -> bool {
        self.walkable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_spawn_despawn() {
        let mut tile = Tile::new('.', true, (0, 0));
        tile.spawn_enemy(1);
        assert!(tile.npc.is_some());
        tile.despawn_enemy();
        assert!(tile.npc.is_none());
    }

    #[test]
    fn test_tile_walkable() {
        let mut tile = Tile::new('.', true, (0, 0));
        assert!(tile.is_walkable());
        tile.set_character();
        assert!(!tile.is_walkable());
        tile.unset_character();
        assert!(tile.is_walkable());
        tile.spawn_enemy(1);
        assert!(!tile.is_walkable());
        tile.despawn_enemy();
        assert!(tile.is_walkable());
    }

    #[test]
    fn test_tile_symbol() {
        let mut tile = Tile::new('.', true, (0, 0));
        assert_eq!(tile.get_symbol(), '.');
        tile.set_character();
        assert_eq!(tile.get_symbol(), '@');
        tile.unset_character();
        assert_eq!(tile.get_symbol(), '.');
        tile.spawn_enemy(1);
        let symbol = tile.get_symbol();
        assert!(symbol != '.');
        tile.despawn_enemy();
        assert_eq!(tile.get_symbol(), '.');
    }
}
