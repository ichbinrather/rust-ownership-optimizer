struct Game {
    players: Vec<String>,
    log: Vec<String>,
}

impl Game {
    fn announce(&mut self) {
        for p in self.players.iter() {
            self.log.push(format!("{p} joined"));
        }
    }
}

fn main() {
    let mut g = Game { players: vec!["x".into(), "y".into()], log: vec![] };
    g.announce();
    println!("{:?}", g.log);
}
