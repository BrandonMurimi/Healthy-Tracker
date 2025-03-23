User Stories Table

| Story ID | User Story                                                                 | Acceptance Criteria                                                                 | Priority (High/Medium/Low) |
|----------|----------------------------------------------------------------------------|-------------------------------------------------------------------------------------|----------------------------|
| US-001   | As a user, I want to track my daily steps so that I can monitor my activity. | Steps are recorded and displayed on the dashboard.                                  | High                       |
| US-002   | As a user, I want to view my fitness data so that I can track my progress.  | Data is displayed in charts and graphs.                                             | High                       |
| US-003   | As a user, I want to set fitness goals so that I can achieve my targets.    | Goals are saved and displayed on the dashboard.                                     | High                       |
| US-004   | As a user, I want to sync data from my wearable device so that I can automatically track my fitness. | Data is synced and updated in real-time.                                            | High                       |
| US-005   | As a user, I want to generate a fitness report so that I can analyze my progress. | Report is generated and downloadable in PDF format.                                 | Medium                     |
| US-006   | As an admin, I want to manage user accounts so that I can add or remove users. | User accounts are updated in the system.                                            | Medium                     |
| US-007   | As a user, I want to receive notifications so that I can stay on track with my fitness goals. | Notifications are sent for overdue goals or milestones.                              | Medium                     |
| US-008   | As a user, I want to analyze fitness trends so that I can understand my progress over time. | Trends are displayed in a user-friendly format (e.g., line charts).                 | Medium                     |
| US-009   | As a system admin, I want user data encrypted with AES-256 so that security compliance is met. | All user data is encrypted during transmission and storage.                         | High                       |
| US-010   | As a system admin, I want the system to handle 1,000 concurrent users so that performance is maintained. | System response time is ≤ 2 seconds under load.                                     | High                       |


2. Product Backlog Creation
Product Backlog Table

| Story ID | User Story                                                                 | Priority (MoSCoW) | Effort Estimate (1–5) | Dependencies       |
|----------|----------------------------------------------------------------------------|-------------------|-----------------------|--------------------|
| US-001   | As a user, I want to track my daily steps so that I can monitor my activity. | Must-have         | 3                     | None               |
| US-002   | As a user, I want to view my fitness data so that I can track my progress.  | Must-have         | 2                     | US-001             |
| US-003   | As a user, I want to set fitness goals so that I can achieve my targets.    | Must-have         | 3                     | US-002             |
| US-004   | As a user, I want to sync data from my wearable device so that I can automatically track my fitness. | Must-have         | 4                     | None               |
| US-005   | As a user, I want to generate a fitness report so that I can analyze my progress. | Should-have       | 5                     | US-002             |
| US-006   | As an admin, I want to manage user accounts so that I can add or remove users. | Should-have       | 3                     | None               |
| US-007   | As a user, I want to receive notifications so that I can stay on track with my fitness goals. | Should-have       | 4                     | US-003             |
| US-008   | As a user, I want to analyze fitness trends so that I can understand my progress over time. | Could-have        | 5                     | US-002             |
| US-009   | As a system admin, I want user data encrypted with AES-256 so that security compliance is met. | Must-have         | 3                     | None               |
| US-010   | As a system admin, I want the system to handle 1,000 concurrent users so that performance is maintained. | Must-have         | 5                     | None               |

Justification for Prioritization
- **Must-have**: Stories critical for the MVP, such as tracking steps (US-001), viewing data (US-002), and setting goals (US-003), align with stakeholder success metrics for usability and core functionality.
- **Should-have**: Stories like generating reports (US-005) and managing users (US-006) enhance the system but are not essential for the MVP.
- **Could-have**: Stories like analyzing trends (US-008) provide additional value but can be deferred to later sprints.
- **Won’t-have**: No stories are classified as "Won’t-have" at this stage.

3. Sprint Planning
Sprint Goal
- **Goal**: Implement core functionality for tracking, viewing, and syncing fitness data to deliver the MVP.

| Story ID | User Story                                                                 | Priority (MoSCoW) | Effort Estimate (1–5) |
|----------|----------------------------------------------------------------------------|-------------------|-----------------------|
| US-001   | As a user, I want to track my daily steps so that I can monitor my activity. | Must-have         | 3                     |
| US-002   | As a user, I want to view my fitness data so that I can track my progress.  | Must-have         | 2                     |
| US-003   | As a user, I want to set fitness goals so that I can achieve my targets.    | Must-have         | 3                     |
| US-004   | As a user, I want to sync data from my wearable device so that I can automatically track my fitness. | Must-have         | 4                     |
| US-009   | As a system admin, I want user data encrypted with AES-256 so that security compliance is met. | Must-have         | 3                     |




Sprint Backlog Table
| Task ID | Task Description                          | Assigned To | Estimated Hours | Status (To Do/In Progress/Done) |
|---------|-------------------------------------------|-------------|------------------|----------------------------------|
| T-001   | Develop API for tracking steps            | Dev Team    | 8                | To Do                            |
| T-002   | Design UI for viewing fitness data        | UI Team     | 6                | To Do                            |
| T-003   | Implement goal-setting functionality      | Dev Team    | 8                | To Do                            |
| T-004   | Develop wearable device sync module       | Dev Team    | 10               | To Do                            |
| T-005   | Implement AES-256 encryption for user data| Dev Team    | 6                | To Do                            |

Sprint Goal Statement
- The sprint focuses on delivering core functionality for tracking, viewing, and syncing fitness data, which is essential for the MVP. This aligns with stakeholder needs for usability and security.

- 4. Documentation & Clarity
Reflection

**Challenges in Prioritization, Estimation, and Alignment**

As the sole stakeholder for this project, I faced several challenges in translating requirements into actionable Agile artifacts, such as user stories, a prioritized backlog, and a sprint plan. These challenges stemmed from the need to balance technical feasibility, resource constraints, and my own expectations as the stakeholder. Below, I reflect on the key challenges I encountered and how I addressed them.

1. Prioritization Challenges
Prioritizing user stories was one of the most difficult aspects of this assignment. As the stakeholder, I had a clear vision of what I wanted the system to achieve, but I also had to consider the technical complexity and effort required for each feature. For example:

Core Functionality vs. Nice-to-Have Features: I struggled to differentiate between "must-have" features (e.g., tracking steps, viewing data) and "should-have" or "could-have" features (e.g., generating reports, analyzing trends). My initial instinct was to prioritize everything as high importance, but I quickly realized that this approach would overwhelm the development team and delay the MVP.

Balancing Stakeholder Needs: Since I was both the stakeholder and the product owner, I had to constantly remind myself to prioritize features that delivered the most value to the end user (me) while staying within the constraints of time and resources.

To address these challenges, I used the MoSCoW prioritization method to categorize user stories into "Must-have," "Should-have," "Could-have," and "Won’t-have." This helped me focus on delivering the MVP first and deferring less critical features to later sprints.

2. Estimation Challenges
Estimating the effort required for each user story was another significant challenge. As someone with limited experience in Agile estimation, I found it difficult to assign story points accurately. For example:

Uncertainty in Technical Complexity: Tasks like syncing data from wearable devices (US-004) or implementing AES-256 encryption (US-009) required research and planning, making it hard to estimate the effort upfront.

Over- or Under-Estimating Effort: I initially underestimated the effort required for some tasks, such as designing the UI for viewing fitness data (T-002), and overestimated others, such as setting fitness goals (US-003).

To improve my estimation process, I broke down each user story into smaller tasks and assigned effort estimates based on the complexity of each task. I also consulted online resources and Agile guides to better understand how to assign story points.

3. Aligning Agile with Stakeholder Needs
As the sole stakeholder, I had to wear multiple hats: defining requirements, prioritizing features, and ensuring that the development plan aligned with my expectations. This dual role created internal resistance and made it difficult to maintain objectivity. For example:

Conflicting Priorities: I often found myself torn between wanting to deliver a feature-rich system and staying within the constraints of a two-week sprint. This internal conflict made it challenging to make decisions that balanced functionality, usability, and feasibility.

Lack of External Feedback: Without input from other stakeholders, I had to rely on my own judgment to validate requirements and prioritize features. This lack of external feedback made it difficult to ensure that the system would meet the needs of potential future users.

To address these challenges, I adopted a user-centric approach by focusing on features that would provide the most value to me as the end user. I also used Agile tools like GitHub Projects to visualize the backlog and track progress, which helped me stay organized and maintain a clear focus on the MVP.

4. Overcoming Internal Resistance
One of the most surprising challenges was overcoming my own resistance to the Agile process. As someone who prefers structured, detailed plans, I initially struggled with the iterative and flexible nature of Agile. For example:

Embracing Uncertainty: Agile requires embracing uncertainty and being open to change, which was difficult for me as I wanted to have everything planned out in advance.

Letting Go of Perfectionism: I had to let go of my perfectionist tendencies and accept that the MVP would not include every feature I envisioned. This was a difficult but necessary step to ensure timely delivery.
