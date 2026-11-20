struct Config {
    banned: Vec<String>,
    min_len: usize,
}

fn make_filters<'a>(cfg: &'a Config) -> Vec<Box<dyn Fn(&str) -> bool + 'a>> {
    vec![
        Box::new(move |w| !cfg.banned.iter().any(|b| b == w)),
        Box::new(move |w| w.len() >= cfg.min_len),
    ]
}

fn main() {
    let cfg = Config { banned: vec!["spam".into(), "junk".into()], min_len: 3 };
    let filters = make_filters(&cfg);
    let words = ["ok", "spam", "hello", "junk", "rust"];
    let kept: Vec<&str> = words.iter().copied().filter(|w| filters.iter().all(|f| f(w))).collect();
    println!("{kept:?}");
}
