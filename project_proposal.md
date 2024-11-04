# Project Proposal

Team members: Ginny Ji (1002492698), Xinyi Gong (1002109826)

## Motivation
Our motivation for this project results from a need within the Rust ecosystem: the absence of a robust, lightweight, scalable, and secure real-time chat application. While chat applications are abundant in other programming languages, there is a **notable lack of open-source Rust-based solutions** that emphasize both security and scalability. This gap represents an opportunity to leverage Rust’s inherent advantages, such as **memory safety, concurrency handling, and high performance**, to create an application that serves as a benchmark for real-time communication.

Building a real-time chat application in Rust is particularly compelling because of the language’s guarantees of **memory safety without a garbage collector**. This unique characteristic enables developers to create high-performance applications that can scale effectively while minimizing common security vulnerabilities. Additionally, our project aims to demonstrate how **Rust’s asynchronous programming model** and efficient memory management can be harnessed to handle multiple concurrent users with minimal latency.

By addressing this gap, our project will not only fill a void in the Rust ecosystem but also showcase the language's suitability for modern, high-performance applications. Real-time communication is essential for collaboration tools, making this project timely and impactful for developers and end-users alike. Furthermore, by contributing this open-source project, we enrich the community's resources and encourage further innovation in Rust web development.

## Objective and Key Features

### Objective
Our primary objective is to develop a **scalable and high-performance real-time chat application** built entirely in Rust. This application will enable users to create rooms, send messages instantly, and monitor other users' online or offline status. By leveraging Rust’s **performance benefits and memory safety guarantees**, we aim to deliver a secure and responsive user experience. This project will serve as a comprehensive case study on how Rust can be used to create robust, real-time communication systems, filling a critical gap in the Rust ecosystem while contributing valuable knowledge and resources to the community.

### Key Features
1. **User Authentication**: Implement secure user sign-up, login, and session management using JWTs to ensure a reliable and protected user experience, preventing unauthorized access.
2. **Chat Room Management**: Allow users to create and join chat rooms for topic-based discussions, facilitating seamless and flexible communication.
3. **Real-Time Messaging**: Use WebSocket technology for instant message delivery, ensuring a lag-free experience within chat rooms.
4. **Presence Detection**: Include online/offline status indicators to enhance user engagement and provide real-time awareness of active participants.
5. **Frontend Integration with Yew**: Build a responsive, accessible, and cross-browser-compatible front-end using Yew. Ensure the UI adheres to modern design principles and is optimized for usability, including features like real-time notifications and an intuitive chat layout.
6. **Scalable Backend**: Design a backend capable of handling multiple concurrent users with low latency, highlighting Rust’s capacity for high scalability and performance.
7. **Extensibility and Community Contributions**: Create modular code with clear documentation to encourage open-source contributions.

### Additional Highlights

1. **Full-Stack Rust Development**:
   The project fills a critical gap in the Rust ecosystem by providing a comprehensive, scalable real-time chat application that leverages Rust’s strengths for both the front end (using **Yew**) and back end. This integration showcases Rust's potential for developing secure, high-performance, full-stack web applications.

2. **Security and JWT-Based Authentication**:
   A major feature is the robust **JWT validation middleware**, which ensures only authorized users access protected routes by validating tokens and securely checking user IDs against the database. With seamless integration of token revocation via a global blacklist and leveraging `Actix Web` and `sqlx` for secure database interactions, as well as `HMAC-SHA256` for token signing, this feature bolsters the security model. It provides reliable, high-performance access control and data protection.

3. **RESTful Architecture**:
   The project follows **RESTful principles**, featuring stateless communication with **resource-based endpoints** and the use of HTTP methods (GET, POST, DELETE) for clear, structured interactions. This makes the app highly interoperable and modular, adhering to modern web standards.

4. **Yew Framework and WebAssembly**:
   Utilizing **Yew** for the front end brings a Rust-native, component-based UI framework with a React-like, developer-friendly architecture that simplifies code maintenance and scalability. Paired with **WebAssembly (Wasm)**, Yew enables near-native execution speed, providing a responsive user experience suitable for complex, real-time applications. This choice fills a niche in Rust projects by showcasing how Yew can handle complex, real-time UIs with robust performance.

5. **Scalability and Deployment**:
   The project integrates **Docker** for consistent containerization and **Fly.io** for straightforward, global deployment. This combination supports scalable, distributed application instances and allows seamless movement between development and production environments, simplifying collaborative work and maintenance.

6. **Modular and Maintainable Codebase**:
   The design follows best practices with a clear **separation of concerns** between route handlers and data models, ensuring the application is easy to extend and adapt. This modular structure encourages reasonable work distribution among team members and aligns with industry standards for clean, maintainable code.


### Optional optimizations
1. Set up a performance monitoring to measure and log latency under various load conditions. Set a performance goal of maintaining sub-100ms message latency under a load of up to 1,000 concurrent users.

2. Employ additional best practices to mitigate common vulnerabilities (e.g., CSRF, XSS) and incorporate TLS for secure data transmission.


### Endpoint Design

Note: All endpoints requiring authentication will be protected using JWT validation middleware to ensure secure access control.

1. **Authentication Endpoints**  
   - POST `/register`
     - **Description**: Registers a new user by creating an account. Include validation rules, such as a minimum password length or disallowed characters.
     - **Request Body**: `{ "username": "string", "password": "string" }`
     - **Response**: Returns `201 Created` with confirmation if successful, or `400 Bad Request` for input errors.
   
   - POST `/login`
     - **Description**: Authenticates a user and issues a JWT token upon successful login.
     - **Request Body**: `{ "username": "string", "password": "string" }`
     - **Response**: `{ "token": "string" }` on success, or `401 Unauthorized` if authentication fails.
   
   - POST `/logout` (authenticated)
     - **Description**: Logs out a user by invalidating their session or updating their status.
     - **Request Header**: `Authorization: Bearer <JWT Token>`
     - **Response**: `200 OK` on successful logout, or `401 Unauthorized` if the user is not authenticated.

2. **Chat Room Management Endpoints**  
   - POST `/rooms` (authenticated)  
     - **Description**: Allows users to create new chat rooms.
     - **Request Body**: `{ "room_name": "string" }`
     - **Response**: Returns `{ "room_id": "int" }` on success, or `400 Bad Request` for input errors.
   
   - GET `/rooms`  
     - **Description**: Retrieves a list of all available chat rooms.
     - **Response**: An array of room objects `[ { "room_id": "int", "room_name": "string" } ]`.

   - POST `/rooms/{room_id}/join` (authenticated)  
     - **Description**: Enables users to join a specific chat room by ID.
     - **Path Parameter**: `room_id` - The identifier of the chat room.
     - **Response**: `200 OK` on success, or `404 Not Found` if the room does not exist.

3. **Real-Time Messaging Endpoints**  
   - **GET `/ws/rooms/{room_id}`** (authenticated)  
     - **Description**: Initiates a WebSocket connection for real-time messaging within a specific chat room.
     - **Path Parameter**: `room_id` - The identifier of the room to join.
     - **WebSocket Messages**:
       - **Client to Server**: JSON format for sending messages: `{ "message": "string" }`
       - **Server to Client**: Broadcasts incoming messages to all participants and updates on user presence.

4. **Presence and Status Endpoints**  
   - GET `/users/presence` (authenticated)  
     - **Description**: Provides the online/offline status of users within a chat room.
     - **Response**: An array of user presence statuses `[ { "username": "string", "status": "online"/"offline" } ]`.

### Database Schema Design

1. `users` Table
   - **Description**: Stores user information and credentials.
   - **Columns**:
     - `user_id` (INTEGER PRIMARY KEY): Unique identifier for each user.
     - `username` (TEXT, UNIQUE, NOT NULL): Username for user identification.
     - `password_hash` (TEXT, NOT NULL): Hashed password for authentication.
     - `created_at` (TIMESTAMP DEFAULT CURRENT_TIMESTAMP): Timestamp indicating when the user was created.

2. `rooms` Table
   - **Description**: Contains information about chat rooms.
   - **Columns**:
     - `room_id` (INTEGER PRIMARY KEY): Unique identifier for each chat room.
     - `room_name` (TEXT, UNIQUE, NOT NULL): The name of the chat room.
     - `user_id` (INTEGER NOT NULL): The ID of the user who created the room.
     - `created_at` (TIMESTAMP DEFAULT CURRENT_TIMESTAMP): Timestamp for when the room was created.

3. `user_rooms` Table
   - **Description**: Represents a many-to-many relationship between users and chat rooms, tracking users who have joined a room.
   - **Columns**:
     - `user_id` (INTEGER, FOREIGN KEY REFERENCES users(user_id)): ID of the user.
     - `room_id` (INTEGER, FOREIGN KEY REFERENCES rooms(room_id)): ID of the room.
     - `joined_at` (TIMESTAMP DEFAULT CURRENT_TIMESTAMP): Timestamp of when the user joined the room.

4. `messages` Table
   - **Description**: Stores messages sent in chat rooms.
   - **Columns**:
     - `message_id` (INTEGER PRIMARY KEY): Unique identifier for each message.
     - `room_id` (INTEGER, FOREIGN KEY REFERENCES rooms(room_id)): ID of the chat room where the message was sent.
     - `user_id` (INTEGER, FOREIGN KEY REFERENCES users(user_id)): ID of the sender.
     - `content` (TEXT, NOT NULL): The actual content of the message.
     - `sent_at` (TIMESTAMP DEFAULT CURRENT_TIMESTAMP): Timestamp of when the message was sent.

5. `user_presence` Table
   - **Description**: Tracks the online/offline status of users.
   - **Columns**:
     - `user_id` (INTEGER, FOREIGN KEY REFERENCES users(user_id)): ID of the user.
     - `last_seen` (TIMESTAMP): The last time the user was active.
     - `status` (TEXT): Indicates if the user is "online" or "offline".

## Tentative Plan

**Development Phases**

- **Week 1: Initial Setup, Database Design, and User Authentication**
  - **Tasks**:
    - *Teammate 1*: Set up the project repository and folder structure, create configuration files, and develop user sign-up and login endpoints with secure password hashing.
    - *Teammate 2*: Design the database schema, including tables for users, chat rooms, and user-room relationships. Implement JWT-based session management and connect authentication logic to the database.
    - **Collaboration**: Review and finalize the database schema, integrate SQLx, and test the user authentication flow end-to-end.
  - **Outcome**: Complete initial project setup, database structure, and a working user authentication system.

- **Week 2: Development of Chat Room Management**
  - **Tasks**:
    - *Teammate 1*: Create API endpoints for creating and managing chat rooms, including CRUD operations.
    - *Teammate 2*: Develop logic for users joining and leaving chat rooms and maintaining user-room relationships in the database.
    - **Collaboration**: Integrate chat room management with database logic and run tests to validate endpoint functionality.
  - **Outcome**: Functional API endpoints for chat room creation and user participation.

- **Week 3: Implementation of Real-Time Messaging**
  - **Tasks**:
    - *Teammate 1*: Set up WebSocket server connections using Actix Web and implement message routing logic within chat rooms.
    - *Teammate 2*: Develop user presence detection (e.g., online/offline status) and implement broadcasting of messages to connected users.
    - **Collaboration**: Test WebSocket server functionality for stable connections and reliable messaging, ensuring presence detection works in real-time.
  - **Outcome**: Fully functional real-time messaging system with user presence indicators.

- **Week 4: Frontend Development with Yew**
  - **Tasks**:
    - *Teammate 1*: Develop the core UI components for user registration, login, and chat room interfaces using Yew.
    - *Teammate 2*: Integrate the Yew frontend with backend APIs for authentication and chat room interactions.
    - **Collaboration**: Collaborate to integrate WebSocket client logic for handling real-time updates and test frontend-backend interactions for consistent behavior.
  - **Outcome**: Interactive frontend with full integration of backend functionalities.

- **Week 5: Final Testing, Deployment, and Documentation**
  - **Tasks**:
    - *Teammate 1*: Prepare a Dockerfile for containerized deployment, verify the container runs correctly, and deploy the app on Fly.io.
    - *Teammate 2*: Configure deployment environment variables and database connectivity, and conduct integration tests to ensure smooth frontend-backend interaction.
    - **Collaboration**: Perform joint load testing to ensure app stability under concurrent use. Draft the README and setup instructions, create architectural diagrams, and record a project demo video.
  - **Outcome**: A fully deployed, optimized chat application with complete project documentation and a demonstration video.

  
