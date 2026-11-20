use std::collections::HashMap;

pub struct Doc {
    pub title: String,
    pub body: String,
}

pub struct Store {
    pub docs: Vec<Doc>,
    pub summaries: HashMap<usize, String>,
    pub computed: usize,
}

impl Store {
    pub fn demo() -> Self {
        let docs = vec![
            Doc { title: "Ownership".into(), body: "Each value has one owner. When the owner goes out of scope, the value is dropped.".into() },
            Doc { title: "Borrowing".into(), body: "References borrow a value without taking ownership. They must not outlive it.".into() },
            Doc { title: "Lifetimes".into(), body: "Lifetimes describe how long references are valid".into() },
        ];
        Store { docs, summaries: HashMap::new(), computed: 0 }
    }

    pub fn title(&self, idx: usize) -> &str {
        &self.docs[idx].title
    }
}
