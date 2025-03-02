
use std::collections::HashMap;

#[derive(Debug)]
struct HealthData {
    steps: u32,
    heart_rate: u32,
    calories_burned: u32,
}

struct HealthTracker {
    data: HashMap<String, HealthData>, // Maps user names to their health data
}

impl HealthTracker {
    // Create a new HealthTracker
    fn new() -> Self {
        HealthTracker {
            data: HashMap::new(),
        }
    }

    // Add health data for a user
    fn add_data(&mut self, user: &str, steps: u32, heart_rate: u32, calories_burned: u32) {
        let health_data = HealthData {
            steps,
            heart_rate,
            calories_burned,
        };
        self.data.insert(user.to_string(), health_data);
    }

    // View health data for a user
    fn view_data(&self, user: &str) -> Option<&HealthData> {
        self.data.get(user)
    }

    // Calculate total steps, heart rate, and calories burned for all users
    fn calculate_totals(&self) -> (u32, u32, u32) {
        let mut total_steps = 0;
        let mut total_heart_rate = 0;
        let mut total_calories_burned = 0;

        for health_data in self.data.values() {
            total_steps += health_data.steps;
            total_heart_rate += health_data.heart_rate;
            total_calories_burned += health_data.calories_burned;
        }

        (total_steps, total_heart_rate, total_calories_burned)
    }
}

fn main() {
    let mut tracker = HealthTracker::new();

    // Add health data for users
    tracker.add_data("Alice", 5000, 80, 300);
    tracker.add_data("Bob", 7000, 90, 450);
    tracker.add_data("Charlie", 3000, 75, 200);

    // View health data for a specific user
    if let Some(data) = tracker.view_data("Alice") {
        println!("Alice's Health Data: {:?}", data);
    } else {
        println!("User not found.");
    }

    // Calculate and display totals
    let (total_steps, total_heart_rate, total_calories_burned) = tracker.calculate_totals();
    println!(
        "Totals - Steps: {}, Heart Rate: {}, Calories Burned: {}",
        total_steps, total_heart_rate, total_calories_burned
    );
}
