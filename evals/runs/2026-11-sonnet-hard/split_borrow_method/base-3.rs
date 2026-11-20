struct Order {
    items: Vec<(String, u32)>,
    discount: u32,
    total: u32,
    log: Vec<String>,
}

impl Order {
    fn add_line(total: &mut u32, log: &mut Vec<String>, discount: u32, name: &str, price: u32) {
        let charged = price.saturating_sub(discount);
        *total += charged;
        log.push(format!("{name}: {charged}"));
    }

    fn checkout(&mut self) {
        for (name, price) in &self.items {
            Self::add_line(&mut self.total, &mut self.log, self.discount, name, *price);
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
