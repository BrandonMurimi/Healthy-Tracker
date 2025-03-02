Health Tracker System
This is a simple health tracking system implemented in Rust. The system allows you to track health data (steps, heart rate, and calories burned) for multiple users and provides functionality to view individual user data and calculate totals across all users.

Features
Add Health Data: Add health data (steps, heart rate, and calories burned) for a specific user.

View Health Data: View the health data for a specific user.

Calculate Totals: Calculate the total steps, heart rate, and calories burned across all users.

Usage
Clone the repository:

bash

cd health-tracker
Run the program:

bash

cargo run
Add Health Data:

rust
tracker.add_data("Alice", 5000, 80, 300);
tracker.add_data("Bob", 7000, 90, 450);
tracker.add_data("Charlie", 3000, 75, 200);
View Health Data:

rust

if let Some(data) = tracker.view_data("Alice") {
    println!("Alice's Health Data: {:?}", data);
} else {
    println!("User not found.");
}
Calculate Totals:

rust

let (total_steps, total_heart_rate, total_calories_burned) = tracker.calculate_totals();
println!(
    "Totals - Steps: {}, Heart Rate: {}, Calories Burned: {}",
    total_steps, total_heart_rate, total_calories_burned
);
Code Structure
HealthData Struct: Represents the health data for a user, including steps, heart rate, and calories burned.

HealthTracker Struct: Manages a collection of health data for multiple users using a HashMap.

HealthTracker Implementation:

new(): Initializes a new HealthTracker.

add_data(): Adds health data for a user.

view_data(): Retrieves health data for a specific user.

calculate_totals(): Calculates the total steps, heart rate, and calories burned across all users.

Example
rust
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
Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

Fork the repository.

Create a new branch (git checkout -b feature/YourFeatureName).

Commit your changes (git commit -m 'Add some feature').

Push to the branch (git push origin feature/YourFeatureName).

Open a Pull Request.



Links
-https://github.com/BrandonMurimi/Healthy-Tracker/commit/b0672e068f66e8e0171017a01ceb3593db030c30
- https://github.com/BrandonMurimi/Healthy-Tracker/commit/eaf9cccd761a70ceadf7a7b51cec8d086e153eec
