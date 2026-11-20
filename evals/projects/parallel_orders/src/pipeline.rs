use std::collections::HashMap;

pub struct Order {
    pub id: u64,
    pub country: String,
    pub items: Vec<(String, u64)>,
}

pub struct Pipeline {
    pub orders: Vec<Order>,
    pub tax_percent: HashMap<String, u64>,
}

impl Pipeline {
    pub fn demo(n: u64) -> Self {
        let countries = ["DE", "TR", "US", "FR"];
        let orders = (0..n)
            .map(|id| Order {
                id,
                country: countries[(id % 4) as usize].to_string(),
                items: (0..(id % 5 + 1)).map(|k| (format!("item{k}"), 100 + id * 3 + k * 7)).collect(),
            })
            .collect();
        let tax_percent = [("DE", 19), ("TR", 20), ("US", 7), ("FR", 20)]
            .into_iter()
            .map(|(c, t)| (c.to_string(), t))
            .collect();
        Pipeline { orders, tax_percent }
    }

    fn total(&self, order: &Order) -> u64 {
        let net: u64 = order.items.iter().map(|(_, price)| price).sum();
        let tax = self.tax_percent.get(&order.country).copied().unwrap_or(0);
        net + net * tax / 100
    }

    /// Returns the total of every order, in the original order.
    pub fn process_all(&self) -> Vec<u64> {
        self.orders.iter().map(|o| self.total(o)).collect()
    }
}
