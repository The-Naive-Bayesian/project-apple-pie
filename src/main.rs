use std::io;

fn main() {
    println!("Collaborative meal planner");

    let mut meals: Vec<String> = Vec::new();

    loop {
        println!("Please input a meal option: ");

        let mut entry = String::new();

        io::stdin()
            .read_line(&mut entry)
            .expect("Unable to read meal option");

        let meal = entry.trim().to_owned();

        println!("\nYou entered: '{meal}'\n");
        meals.push(meal);

        println!("So far you have entered the following meals:");
        for item in &meals {
            println!("\t{item}");
        }
        println!()
    }
}
