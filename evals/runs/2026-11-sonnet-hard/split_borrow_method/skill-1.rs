struct Order {
    items: Vec<(String, u32)>,
    discount: u32,
    total: u32,
    log: Vec<String>,
}

impl Order {
    fn add_line(&mut self, name: &str, price: u32) {
        let charged = price.saturating_sub(self.discount);
        self.total += charged;
        self.log.push(format!("{name}: {charged}"));
    }

    fn checkout(&mut self) {
        let items = std::mem::take(&mut self.items);
        for (name, price) in &items {
            self.add_line(name, *price);
        }
        self.items = items;
    }
}

fn main() {
    let mut order = Order {
        items: vec![("tea".into(), 3), ("cake".into(), 5), ("pie".into(), 4)],
        discount: 1,
        total: 0,
        log: Vec::new(),
    };
    order.checkout();
    println!("{} {:?} {}", order.total, order.log, order.items.len());
}
