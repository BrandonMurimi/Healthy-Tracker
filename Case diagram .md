The use Case Diagram
i used marmaid js to Draw the diagram
The link of the picture
![Screenshot 2025-03-16 205121](https://github.com/user-attachments/assets/310c3ff2-590e-462f-b51f-4721a1d149b4)

The coded is below for the mermed diagram
graph TD
    %% Actors
    User[User] -->|Tracks Fitness Data| TrackFitness
    User -->|Views Fitness Data| ViewFitness
    User -->|Sets Fitness Goals| SetGoals
    Admin[Admin] -->|Manages Users| ManageUsers
    Admin -->|Generates Reports| GenerateReports
    WearableDevice[Wearable Device] -->|Syncs Data| SyncData
    System[System] -->|Sends Notifications| SendNotifications
    System -->|Analyzes Trends| AnalyzeTrends

    %% Use Cases
    TrackFitness[Track Fitness Data] -->|Includes| SyncData
    ViewFitness[View Fitness Data] -->|Extends| GenerateReports
    SetGoals[Set Fitness Goals] -->|Depends on| ViewFitness
    AnalyzeTrends[Analyze Trends] -->|Depends on| ViewFitness
    ManageUsers[Manage Users] -->|Updates| System
    GenerateReports[Generate Reports] -->|Uses| ViewFitness
    SyncData[Sync Data] -->|Updates| TrackFitness
    SendNotifications[Send Notifications] -->|Notifies| User

    %% Styles
    style User fill:#f9f,stroke:#333,stroke-width:2px
    style Admin fill:#f9f,stroke:#333,stroke-width:2px
    style WearableDevice fill:#f9f,stroke:#333,stroke-width:2px
    style System fill:#bbf,stroke:#333,stroke-width:2px
    style TrackFitness fill:#9f9,stroke:#333,stroke-width:2px
    style ViewFitness fill:#9f9,stroke:#333,stroke-width:2px
    style SetGoals fill:#9f9,stroke:#333,stroke-width:2px
    style AnalyzeTrends fill:#9f9,stroke:#333,stroke-width:2px
    style ManageUsers fill:#9f9,stroke:#333,stroke-width:2px
    style GenerateReports fill:#9f9,stroke:#333,stroke-width:2px
    style SyncData fill:#9f9,stroke:#333,stroke-width:2px
    style SendNotifications fill:#9f9,stroke:#333,stroke-width:2px


Explanation of the Diagram
Key Actors and Their Roles
User:

Tracks fitness data (e.g., steps, heart rate, calories).

Views fitness data.

Sets fitness goals.

Admin:

Manages user accounts (e.g., adding or removing users).

Generates reports for analysis.

Wearable Device:

Syncs fitness data with the system.

System:

Sends notifications (e.g., reminders, alerts).

Analyzes fitness trends.

Use Cases
Track Fitness Data:

Allows users to track their fitness metrics.

Includes the Sync Data use case to sync data from wearable devices.

View Fitness Data:

Allows users to view their fitness data.

Extends to the Generate Reports use case.

Set Fitness Goals:

Allows users to set fitness goals.

Depends on the View Fitness Data use case.

Analyze Trends:

Analyzes fitness trends over time.

Depends on the View Fitness Data use case.

Manage Users:

Allows admins to manage user accounts.

Updates the System.

Generate Reports:

Generates fitness reports for users and admins.

Uses the View Fitness Data use case.

Sync Data:

Syncs fitness data from wearable devices.

Updates the Track Fitness Data use case.

Send Notifications:

Sends notifications (e.g., reminders, alerts) to users.

Managed by the System.

Relationships Between Actors and Use Cases
Generalization:

User and Admin are both actors who interact with the System.

Include:

Track Fitness Data includes the Sync Data use case.

Extend:

View Fitness Data extends to the Generate Reports use case.

Dependency:

Set Fitness Goals and Analyze Trends depend on the View Fitness Data use case.

How the Diagram Addresses Stakeholder Concerns
Users:

The Track Fitness Data and View Fitness Data use cases address their need for tracking and viewing fitness metrics.

The Set Fitness Goals use case allows them to set and achieve fitness goals.

Admins:

The Manage Users and Generate Reports use cases address their need for user management and data analysis.

Wearable Devices:

The Sync Data use case ensures seamless integration with wearable devices.

System:

The Send Notifications use case ensures timely reminders and alerts, addressing the user’s concern for staying on track with their fitness goals.

The Analyze Trends use case provides insights into fitness progress, addressing the admin’s concern for data-driven decision-making.

Deliverables
Use Case Diagram:

Use the provided Mermaid code to generate the diagram.

Save it as an image or embed it in your documentation.

Written Explanation:

Include the explanation of actors, use cases, relationships, and stakeholder concerns in your documentation.





Below is a test case table for the Health Tracker System to validate functional requirements and non-functional requirements. The table includes 8+ test cases and 2 non-functional test scenarios.

Test Case Table
Functional Test Cases
Test Case ID	Requirement ID	Description	Steps	Expected Result	Actual Result	Status (Pass/Fail)
TC-001	FR-001	User adds fitness data	1. Log in. 2. Select "Add Data". 3. Input data. 4. Submit.	Data is saved successfully.		
TC-002	FR-002	User views fitness data	1. Log in. 2. Select "View Data".	Data is displayed.		
TC-003	FR-003	User sets fitness goals	1. Log in. 2. Select "Set Goals". 3. Input goals. 4. Submit.	Goals are saved successfully.		
TC-004	FR-004	User syncs data from wearable	1. Log in. 2. Select "Sync Data". 3. Confirm sync.	Data is synced successfully.		
TC-005	FR-005	User generates a report	1. Log in. 2. Select "Generate Report".	Report is displayed.		
TC-006	FR-006	Admin manages users	1. Log in as admin. 2. Select "Manage Users". 3. Add/update user.	User account is updated.		
TC-007	FR-007	System analyzes trends	1. Log in. 2. Select "Analyze Trends".	Trends are displayed.		
TC-008	FR-008	System sends notifications	1. Log in. 2. Enable notifications. 3. Trigger notification (e.g., overdue goal).	Notification is sent.		
Non-Functional Test Scenarios
Performance Test
Scenario: Simulate 1,000 concurrent users adding fitness data.

Steps:

Use a load testing tool (e.g., JMeter) to simulate 1,000 concurrent users.

Each user adds fitness data (e.g., steps, heart rate, calories).

Expected Result: The system processes all requests with a response time ≤ 2 seconds.

Actual Result: [To be filled after testing]

Status: [Pass/Fail]

Security Test
Scenario: Verify data encryption during transmission.

Steps:

Use a network monitoring tool (e.g., Wireshark) to capture data packets.

Add fitness data and sync with a wearable device.

Expected Result: All transmitted data is encrypted (e.g., using HTTPS).

Actual Result: [To be filled after testing]

Status: [Pass/Fail]

Deliverables
Test Case Table:

Include the table in your documentation (e.g., TEST_CASES.md).

Update the Actual Result and Status columns after testing.

Non-Functional Test Scenarios:

Include the performance and security test scenarios in your documentation.










