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
            symbol: symbol,
            walkable: walkable,
            //description: description.to_string(),
            //item: None,
            npc: None,
            position: position,
        }
    }

    pub fn spawn_enemy(&mut self) {
        let enemy = HostileEnemy::new((self.position.0, self.position.1));
        self.npc = Some(enemy);
        self.symbol = match self.npc.as_ref().unwrap().get_name() {
            "Goblin" => 'G',
            "Slime" => 'S',
            _ => 'E',
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

    pub fn get_enemy(&self) -> Option<&HostileEnemy> {
        self.npc.as_ref()
    }

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