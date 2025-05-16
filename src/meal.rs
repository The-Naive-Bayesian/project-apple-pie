use std::collections::HashMap;

pub struct MealOption {
    pub name: String,
    pub votes: i64
}

pub struct MealOptionTracker {
    pub options: HashMap<String, MealOption>
}

impl MealOptionTracker {
    pub fn new() -> Self {
        Self {
            options: HashMap::new(),
        }
    }
    pub fn add(&mut self, name: &String) {
        self.options.entry(name.clone()).and_modify(|e| e.votes += 1).or_insert(MealOption {name: name.clone(), votes: 0});
    }
}
