use std::collections::HashMap;

use crate::player::Player;

pub struct World {
    pub players: Vec<Player>,
    pub log: Vec<String>,
    pub heals_given: HashMap<u32, u32>,
    pub turn: u32,
}

impl World {
    pub fn demo() -> Self {
        World {
            players: vec![
                Player::new(1, "Ada", 100, 100),
                Player::new(2, "Bo", 12, 80),
                Player::new(3, "Cy", 0, 90),
                Player::new(4, "Di", 20, 120),
            ],
            log: Vec::new(),
            heals_given: HashMap::new(),
            turn: 0,
        }
    }

    pub fn log_event(&mut self, msg: String) {
        self.log.push(format!("[turn {}] {msg}", self.turn));
    }

    pub fn heal(&mut self, id: u32, amount: u32) {
        if let Some(p) = self.players.iter_mut().find(|p| p.id == id) {
            p.hp = (p.hp + amount).min(p.max_hp);
        }
        *self.heals_given.entry(id).or_insert(0) += 1;
    }

    pub fn report(&self) -> String {
        let mut out = String::new();
        for p in &self.players {
            let heals = self.heals_given.get(&p.id).copied().unwrap_or(0);
            out.push_str(&format!("{} hp={}/{} heals={}\n", p.name, p.hp, p.max_hp, heals));
        }
        for line in &self.log {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}
