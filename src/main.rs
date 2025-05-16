use crate::meal::{MealOptionTracker};
use crate::action::*;

pub mod meal;

fn main() {
    println!("Welcome to the Swordmedia Solutions Collaborative Meal Planner (SSCMP)!");

    let mut tracker = MealOptionTracker::new();

    loop {
        match get_action() {
            Action::AddMeal => { 
                add_meal(&mut tracker);
            }
            Action::Vote => { 
                vote(&mut tracker); 
            }
            Action::InvalidAction => {
                println!("Goodbye!");
                break;
            }
        }
    }
}

pub mod action {
    use std::io;
    use crate::meal::MealOptionTracker;

    pub enum Action {
        AddMeal,
        Vote,
        InvalidAction,
    }

    pub fn get_action() -> Action {
        println!("What would you like to do?");
        println!("Press \"1\" to add a new meal option, \"2\" to vote, or any other button to exit:");

        let mut entry = String::new();
        io::stdin()
            .read_line(&mut entry)
            .expect("Unable to read choice");

        let action = entry.trim().to_owned();

        match action.as_str() {
            "1" => Action::AddMeal,
            "2" => Action::Vote,
            _ => Action::InvalidAction
        }
    }

    pub fn add_meal(tracker: &mut MealOptionTracker) {
        println!("\nWhat meal would you like to add?");

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

    pub fn vote(tracker: &mut MealOptionTracker) {
        respond_to_not_implemented();
    }

    pub fn respond_to_not_implemented() {
        println!("\nSorry, I can't do that yet :(\n");
    }
}
