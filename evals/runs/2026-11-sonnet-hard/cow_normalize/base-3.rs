use std::borrow::Cow;
use std::fmt::Write;

fn normalize(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))
    } else {
        Cow::Borrowed(s)
    }
}

fn main() {
    let mut total = 0;
    let mut underscores = 0;
    let mut w = String::new();
    for i in 0..200_000 {
        w.clear();
        if i % 100 == 0 {
            write!(w, "a b {i}").unwrap();
        } else {
            write!(w, "w{i}").unwrap();
        }
        let n = normalize(&w);
        total += n.len();
        underscores += n.bytes().filter(|&b| b == b'_').count();
    }
    println!("{total} {underscores}");
}
