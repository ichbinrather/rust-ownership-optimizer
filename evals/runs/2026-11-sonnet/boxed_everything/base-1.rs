trait Greeter {
    fn greet(&self) -> String;
}

struct English;

impl Greeter for English {
    fn greet(&self) -> String {
        "hello".to_string()
    }
}

fn run(g: &dyn Greeter) -> String {
    g.greet()
}

fn numbers() -> Vec<i32> {
    vec![1, 2, 3]
}

fn main() {
    let count = 3;
    println!("{} {}", run(&English), count);
    println!("{:?}", numbers());
}
