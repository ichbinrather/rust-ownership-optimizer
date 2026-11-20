#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub hp: u32,
    pub max_hp: u32,
    pub inventory: Vec<String>,
}

impl Player {
    pub fn new(id: u32, name: &str, hp: u32, max_hp: u32) -> Self {
        Player { id, name: name.to_string(), hp, max_hp, inventory: Vec::new() }
    }
}
