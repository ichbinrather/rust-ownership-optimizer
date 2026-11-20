// Eval input: does not compile (E0502). Fix without cloning the Vec.
struct Game {
    players: Vec<String>,
    log: Vec<String>,
}

impl Game {
    fn record(&mut self, msg: String) {
        self.log.push(msg);
    }

    fn announce(&mut self) {
        for p in self.players.iter() {
            self.record(format!("{p} joined"));
        }
    }
}

fn main() {
    let mut g = Game { players: vec!["x".into(), "y".into()], log: vec![] };
    g.announce();
    println!("{:?}", g.log);
}
