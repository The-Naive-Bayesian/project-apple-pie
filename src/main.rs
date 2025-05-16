use std::collections::HashMap;
use std::io;

fn main() {
    println!("Collaborative meal planner");

    let mut tracker = MealOptionTracker {
        options: HashMap::new(),
    };

    loop {
        println!("Please input a meal option: ");

        let mut entry = String::new();

        io::stdin()
            .read_line(&mut entry)
            .expect("Unable to read meal option");

        let meal_name = entry.trim().to_owned();

        println!("\nYou entered: '{meal_name}'\n");

        tracker.add(&meal_name);

        println!("So far you have entered the following meals:");
        for (key, option) in &tracker.options {
            let votes = option.votes;
            println!("\t{key} ({votes} votes)");
        }
        println!()
    }
}

struct MealOption {
    name: String,
    votes: i64
}

struct MealOptionTracker {
    options: HashMap<String, MealOption>
}

impl MealOptionTracker {
    fn add(&mut self, name: &String) {
        self.options.entry(name.clone()).and_modify(|e| e.votes += 1).or_insert(MealOption {name: name.clone(), votes: 0});
    }
}
