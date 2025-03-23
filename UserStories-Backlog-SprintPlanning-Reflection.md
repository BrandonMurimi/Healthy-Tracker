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
1. **Prioritization**:
   - Balancing stakeholder needs with technical feasibility was challenging. For example, ensuring that security (US-009) and performance (US-010) were prioritized without delaying core functionality required careful consideration.

2. **Estimation**:
   - Estimating effort for tasks like syncing data (US-004) was difficult due to uncertainties in integrating with wearable device APIs.

3. **Alignment with Stakeholder Needs**:
   - Aligning user stories with stakeholder needs (e.g., tracking steps, setting goals) required iterative feedback and validation, especially since I am the sole stakeholder at this stage.
   - 
