
## Introduction
Healthy Tracker
### Project Title
Healthy Tracker

### Domain
Healthy Tracker

### Problem Statement
Many individuals struggle to maintain a consistent fitness routine due to a lack of real-time feedback and personalized insights.
 This system aims to provide users with a tool to track their Healthy Tracker activities, monitor progress, and receive actionable recommendations.

### Individual Scope
The system will focus on the following features:
- Real-time tracking of fitness metrics (steps, heart rate, calories burned, distance).
- Integration with wearable devices (e.g., smartwatches).
- Personalized fitness recommendations based on user data.
- A dashboard for visualizing progress and trends.

Feasibility is justified by the availability of wearable device APIs, cloud computing resources, and modern web development frameworks.

---

## C4 Diagrams

### Level 1: System Context Diagram
![System Context Diagram](images/system-context-diagram.png)
- **Actors**: User, Wearable Device
- **System**: Real-Time Fitness Tracker
- **External Systems**: Wearable Device APIs, Cloud Hosting

#### Description
The System Context Diagram shows the high-level interaction between the system, its users, and external systems.
The primary actors are the **User** and the **Wearable Device**.
 The system interacts with external APIs (e.g., Fitbit, Garmin) to fetch fitness data and uses cloud hosting for deployment.

---

### Level 2: Container Diagram
![Container Diagram](images/container-diagram.png)
- **Frontend**: React.js Application
- **Backend**: Node.js Server
- **Database**: MongoDB
- **External APIs**: Wearable Device APIs

#### Description
The Container Diagram breaks down the system into high-level containers.
 The **Frontend** is a React.js application that provides the user interface.
The **Backend** is a Node.js server that handles business logic and communicates with the **Database** (MongoDB) and **External APIs** (wearable device APIs).

---

### Level 3: Component Diagram
![Component Diagram](images/component-diagram.png)
- **Frontend Components**:
  - Dashboard
  - Activity Tracker
  - User Profile
- **Backend Components**:
  - Authentication Service
  - Data Processing Service
  - Recommendation Engine
- **Database Components**:
  - User Data Collection
  - Activity Logs

#### Description
The Component Diagram shows the internal components of each container.
 The **Frontend** includes components like the Dashboard, Activity Tracker, and User Profile.
 The **Backend** includes components like the Authentication Service, Data Processing Service, and Recommendation Engine. The **Database** stores user data and activity logs.

---

### Level 4: Code Diagram (Optional)
![Code Diagram](images/code-diagram.png)
- **Frontend Code**: React Components (e.g., `Dashboard.js`, `ActivityTracker.js`)
- **Backend Code**: Node.js Modules (e.g., `auth.js`, `dataProcessor.js`)
- **Database Code**: MongoDB Schemas (e.g., `UserSchema`, `ActivitySchema`)

#### Description
The Code Diagram provides a low-level view of the code structure. The **Frontend** uses React components like `Dashboard.js` and `ActivityTracker.js`. The **Backend** uses Node.js modules like `auth.js` and `dataProcessor.js`. The **Database** uses MongoDB schemas like `UserSchema` and `ActivitySchema`.
