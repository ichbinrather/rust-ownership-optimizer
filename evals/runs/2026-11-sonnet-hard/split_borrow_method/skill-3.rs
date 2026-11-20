struct Order {
    items: Vec<(String, u32)>,
    discount: u32,
    total: u32,
    log: Vec<String>,
}

impl Order {
    fn add_line(&mut self, name: &str, price: u32) {
        Self::charge(self.discount, &mut self.total, &mut self.log, name, price);
    }

    // Takes disjoint fields so callers can borrow `items` at the same time.
    fn charge(discount: u32, total: &mut u32, log: &mut Vec<String>, name: &str, price: u32) {
        let charged = price.saturating_sub(discount);
        *total += charged;
        log.push(format!("{name}: {charged}"));
    }

    fn checkout(&mut self) {
        for (name, price) in &self.items {
            Self::charge(self.discount, &mut self.total, &mut self.log, name, *price);
        }
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
