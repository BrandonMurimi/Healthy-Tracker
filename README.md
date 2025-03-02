**Health Tracker System**
A simple Rust program to track health data (steps, heart rate, and calories burned) for multiple users.

Features
Add health data for users.

View health data for a specific user.

Calculate totals (steps, heart rate, calories burned) across all users.

 Uses Clone the repository:

bash (insert the link)
cd health-tracker
Run the program:

bash

cargo run
The program will:

Add health data for three users: Alice, Bob, and Charlie.

Display Alice's health data.

Calculate and print the totals for all users.
**
**Example Output**
**
Alice's Health Data: HealthData { steps: 5000, heart_rate: 80, calories_burned: 300 }
Totals - Steps: 15000, Heart Rate: 245, Calories Burned: 950
Code Overview
HealthData Struct: Stores steps, heart rate, and calories burned.

HealthTracker Struct: Manages health data for multiple users using a HashMap.

Functions:

add_data(): Adds health data for a user.

view_data(): Retrieves health data for a specific user.

calculate_totals(): Calculates totals across all users.


Links
-https://github.com/BrandonMurimi/Healthy-Tracker/commit/b0672e068f66e8e0171017a01ceb3593db030c30
- https://github.com/BrandonMurimi/Healthy-Tracker/commit/eaf9cccd761a70ceadf7a7b51cec8d086e153eec
