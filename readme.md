## Project Structure

```graphql
migrations/          
├── 0001_create_users.sql           # SQL migration for creating the users table
└── 0002_create_rooms_and_user_rooms.sql # SQL migration for creating chat rooms and user-room relationship tables
src/
├── config/                         # Configuration-related files, including state management and app settings
│   ├── mod.rs                      # Module entry point for the config folder
│   └── state.rs                    # Manages the application state and configurations
├── middleware/                     # Middleware implementations for handling request processing
│   ├── auth_middleware.rs          # Middleware for JWT-based authentication
│   └── mod.rs                      # Module entry point for middleware
├── models/                         # Data models representing database structures and entities
│   ├── mod.rs                      # Module entry point for models
│   ├── claim.rs                    # Struct for JWT claims
│   ├── user.rs                     # Model definition for user-related data
│   ├── room.rs                     # Model for chat room data
│   └── user_room.rs                # Model for user-room relationships
├── routes/                         # Handlers for different application routes
│   ├── auth.rs                     # Route handlers for authentication (e.g., register, login)
│   ├── room.rs                     # Route handlers for chat room creation and management
│   ├── test_routes.rs              # Route for testing middleware functionality
│   └── mod.rs                      # Module entry point for exporting all routes
├── main.rs                         # Main application entry point with Actix Web server setup
```



## APIs

### Authentication

1. **POST `/register`**
   - **Summary**: Register a new user account.
   - **Description**: Creates a new user in the system with a unique username and a hashed password.
   - **Request Body**:
     ```json
     {
       "username": "string",
       "password": "string"
     }
     ```
   - **Responses**:
     - `201 Created`: User successfully registered.
     - `400 Bad Request`: Invalid input data, such as missing or non-compliant fields.
   - **Security**: None (public access).

2. **POST `/login`**
   - **Summary**: Authenticate a user and issue a JWT token.
   - **Description**: Verifies the provided credentials and returns a JWT token for session management.
   - **Request Body**:
     ```json
     {
       "username": "string",
       "password": "string"
     }
     ```
   - **Responses**:
     - `200 OK`: Returns a JSON object containing the JWT token.
       ```json
       {
         "token": "string"
       }
       ```
     - `401 Unauthorized`: Incorrect username or password.
   - **Security**: None (public access).

3. **POST `/logout`**
   - **Summary**: Log out a user and invalidate their session.
   - **Description**: Ends the current session by invalidating the JWT token.
   - **Request Headers**:
     - `Authorization: Bearer <JWT Token>`
   - **Responses**:
     - `200 OK`: User successfully logged out.
     - `401 Unauthorized`: Token is missing or invalid.
   - **Security**: JWT token required.

### Test

1. **GET `/test-protected`**
   - **Summary**: Test route for verifying middleware and authentication.
   - **Description**: Checks whether the user is properly authenticated using `AuthMiddleware`.
   - **Request Headers**:
     - `Authorization: Bearer <JWT Token>`
   - **Responses**:
     - `200 OK`: User is authenticated.
     - `401 Unauthorized`: Authentication failed due to a missing or invalid token.
   - **Security**: JWT token required.

### Chat Room Management

1. **POST `/rooms`**
   - **Summary**: Create a new chat room.
   - **Description**: Allows an authenticated user to create a new chat room.
   - **Request Body**:
     ```json
     {
       "room_name": "string"
     }
     ```
   - **Responses**:
     - `201 Created`: Room successfully created.
       ```json
       {
         "room_id": "int"
       }
       ```
     - `400 Bad Request`: Invalid input data or duplicate room name.
   - **Security**: JWT token required.

2. **POST `/rooms/{room_id}/join`**
   - **Summary**: Join a specific chat room.
   - **Description**: Enables an authenticated user to join an existing chat room by providing its ID.
   - **Path Parameters**:
     - `room_id` (integer): The unique identifier of the chat room to join.
   - **Responses**:
     - `200 OK`: Successfully joined the room.
     - `404 Not Found`: Room with the specified ID does not exist.
   - **Security**: JWT token required.

3. **GET `/rooms`**
   - **Summary**: Retrieve a list of available chat rooms.
   - **Description**: Returns all existing chat rooms available for the authenticated user to browse or join.
   - **Responses**:
     - `200 OK`: A JSON array of chat rooms.
       ```json
       [
         {
           "room_id": "int",
           "room_name": "string"
         }
       ]
       ```
   - **Security**: JWT token required.

