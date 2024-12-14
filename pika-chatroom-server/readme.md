## Project Structure

```graphql
migrations/        
├── 0001_create_users.sql                # SQL migration (SQLite) for creating the users table
└── 0002_create_rooms_and_user_rooms.sql # SQL migration (SQLite) for creating chat rooms and user-room relationship tables
src/
├── config/                              # Configuration-related files, including state management and app settings
│   ├── mod.rs                           # Module entry point for the config folder
│   └── state.rs                         # Manages the application state and configurations
├── middleware/                          # Middleware implementations for handling request processing
│   ├── auth_middleware.rs               # Middleware for JWT-based authentication
│   └── mod.rs                           # Module entry point for middleware
├── models/                              # Data models representing database structures and entities
│   ├── mod.rs                           # Module entry point for models
│   ├── claim.rs                         # Struct for JWT claims
│   ├── response.rs                      # Structs for standardized response
│   ├── user.rs                          # Model definition for user-related data
│   ├── room.rs                          # Model for chat room data
│   ├── presence.rs                      # Define presence messages, e.g., SetUserOnline, SetUserOffline
│   └── user_room.rs                     # Model for user-room relationships
├── routes/                              # Handlers for different application routes
│   ├── auth.rs                          # Route handlers for authentication (e.g., register, login)
│   ├── room.rs                          # Route handlers for chat room creation and management
│   ├── test_routes.rs                   # Route for testing middleware functionality
│   └── mod.rs                           # Module entry point for exporting all routes
├── websockets/                          # WebSocket handlers for real-time chat functionality
│   ├── chat_session.rs                  # WebSocket handler for individual chat sessions
│   └── mod.rs                           # Module entry point for WebSocket handling
├── main.rs                              # Main application entry point with Actix Web server setup
```

## Steps to Run the Project

1. **Prepare the Environment**:
   - Ensure that you have `Rust`, `Cargo`, and `SQLx CLI` installed. If not, install them using:

     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     cargo install sqlx-cli --no-default-features --features sqlite

2. **Set Up Environment Variables**:

   - Create a `.env` file in the project root directory with the following content:

     ```env
     DATABASE_URL=sqlite:./chat_app.db
     ```

3. **Setup Database**:

   - Create the SQLite database file and set the correct permissions:

     ```bash
     touch chat_app.db
     chmod 664 chat_app.db
     ```

   - Run the SQL migrations to set up or update the database schema:

     ```bash
     sqlx migrate run
     ```

   - This will execute the migration scripts in the `migrations/` directory.

4. **Build the Project**:

   - Compile the project and download all necessary dependencies:

     ```bash
     cargo build
     ```

5. **Start the Server**:

   - Run the server with logging enabled to see detailed logs:

     ```bash
     RUST_LOG=info cargo run
     ```

### Additional Notes

- **Reset Database for Development**:
  - To reset the database, delete the `chat_app.db` file and re-run migrations:

    ```bash
    rm chat_app.db
    sqlx migrate run
    ```

## Accessing Swagger API Documentation

1. **Open Swagger UI**:

   - Once the server is running, open a browser and navigate to:

     ```bash
     http://127.0.0.1:8080/swagger-ui/
     ```

   - The Swagger UI provides a user-friendly interface to explore and interact with all documented endpoints, including Authentication, Test, and Chat Room Management APIs.

2. **Using the Swagger Interface**:

   - Expand each endpoint to view details about parameters, request body structure, responses, and authentication requirements.
   - You can test each API directly in Swagger by entering parameters, headers (e.g., JWT tokens), and request bodies, and clicking **Execute**.

3. **Download OpenAPI JSON Specification**:

   - You can also view or download the raw OpenAPI JSON specification from:

     ```bash
     http://127.0.0.1:8080/api-doc/openapi.json
     ```

---

## Steps to Test APIs

**Note**: You can use [Swagger UI](http://127.0.0.1:8080/swagger-ui/) for a visual interface to send API requests and explore endpoints. The instructions below are for testing APIs using `curl` from the command line.

### Common Setup: Register and Obtain JWT Token for Authentication

For protected endpoints, you’ll need a registered user and a JWT token obtained via login. Follow these steps to register and obtain the token:

1. **Register a User**:
   - Send a `POST` request to register a new user with the username and password.

   ```bash
   curl -X POST http://127.0.0.1:8080/api/register \
        -H "Content-Type: application/json" \
        -d '{"username": "testuser1", "password": "password123"}'
   ```

   Replace `"testuser1"` and `"password123"` with your preferred credentials. This should return a `201 Created` response, indicating the user was created successfully.

2. **Login to Obtain a JWT Token**:
   - Use the registered username and password to log in, then store the token in a variable for easy reuse:

   ```bash
   TOKEN=$(curl -X POST http://127.0.0.1:8080/api/login \
        -H "Content-Type: application/json" \
        -d '{"username": "testuser2", "password": "password123"}' | sed -n 's/.*"token":"\([^"]*\)".*/\1/p')
   ```

   This command will log in and extract the token, storing it in the `$TOKEN` variable for reuse in further requests.

---

Now that you have a registered user and an authentication token, proceed to test each endpoint as needed. Use `$TOKEN` in requests requiring authentication. 

### Steps to Test the Auth Endpoints

1. **Clean Up the Database**:

   - Open the SQLite CLI and connect to your database file:

     ```bash
     sqlite3 chat_app.db
     ```

   - Run these commands to delete tables and reset the `AUTOINCREMENT` counter:

     ```sql
     DELETE FROM users;
     DELETE FROM sqlite_sequence WHERE name = 'users';
     DELETE FROM rooms;
     DELETE FROM sqlite_sequence WHERE name = 'rooms';
     DELETE FROM user_rooms;
     DELETE FROM sqlite_sequence WHERE name = 'user_rooms';
     ```

   - Exit the SQLite CLI:

     ```bash
     .exit
     ```

2. **Test the Register Endpoint**:

   - Send a `POST` request to `http://127.0.0.1:8080/api/register` with a JSON payload containing a username and password.
   - Use `curl`:

     ```bash
     curl -X POST http://127.0.0.1:8080/api/register \
          -H "Content-Type: application/json" \
          -d '{"username": "testuser1", "password": "password123"}'
     ```

   - Verify that you receive a `201 Created` response, indicating the user was created successfully.

3. **Verify User Creation in Database**:

   - Reopen the SQLite CLI and check that the user was created:

     ```sql
     SELECT * FROM users WHERE username = 'testuser1';
     ```

   - Ensure that `testuser` appears in the results.

4. **Test the Login Endpoint**:

   - Send a `POST` request to `http://127.0.0.1:8080/api/login` with the same username and password.
   - Use `curl`:

     ```bash
     curl -X POST http://127.0.0.1:8080/api/login \
          -H "Content-Type: application/json" \
          -d '{"username": "testuser1", "password": "password123"}'
     ```

   - Confirm that you receive a `200 OK` response with a token in the response body. Save this token for the logout test.

5. **Test the Logout Endpoint**:

   - Use the token obtained from the login response to send a `POST` request to `http://127.0.0.1:8080/api/logout`.
   
   - With `curl`, run:
   
     ```bash
     curl -X POST http://127.0.0.1:8080/api/logout \
          -H "Authorization: Bearer $TOKEN"
     ```
   
   - Check that you receive a `200 OK` response with a message indicating a successful logout.

---

### Steps to Test the Middleware

1. **Test with a Valid Token**:

   - Use `curl` to send a request with a valid token to a protected route:

     ```bash
     curl -H "Authorization: Bearer $TOKEN" http://127.0.0.1:8080/api/test-protected
     ```

   - **Expected Result**: You should receive a `200 OK` response with the message from the protected route.
   
2. **Test with an Invalid Token**:

   - Use `curl` to send a request with an invalid token:

     ```bash
     curl -H "Authorization: Bearer $TOKEN" http://127.0.0.1:8080/api/test-protected
     ```

   - **Expected Result**: You should receive a `401 Unauthorized` response.

3. **Test Without a Token**:

   - Use `curl` to send a request without an `Authorization` header:

     ```bash
     curl http://127.0.0.1:8080/api/test-protected
     ```

   - **Expected Result**: You should receive a `401 Unauthorized` response.

---

### Steps to Test Chat Room Management APIs

1. **Test the Create Room Endpoint (`POST /api/rooms`)**:

   - **Description**: Send a request to create a new chat room as an authenticated user.
   - **Precondition**: Ensure you have a valid JWT token from the login endpoint.

   ```bash
   curl -X POST http://127.0.0.1:8080/api/rooms \
        -H "Authorization: Bearer $TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"room_name": "testroom1"}'
   ```

   - **Expected Result**:
     - On success, you should receive a `201 Created` response with a JSON body containing the full room data:

       ```json
       {
         "room_id": 1,
         "room_name": "testroom1",
         "user_id": <your_user_id>
       }
       ```

     - If the room name already exists, you should receive a `400 Bad Request` response indicating a duplicate room name.

2. **Verify Room Creation in the Database**:

   - **Description**: Check that the room was correctly created by querying the database.
   - **Command**:

     ```sql
     SELECT * FROM rooms WHERE room_name = 'testroom1';
     ```

   - **Expected Result**: You should see an entry in the `rooms` table with the name `testroom1` and an associated `room_id`.

3. **Test the Retrieve Rooms Endpoint (`GET /api/rooms`)**:

   - **Description**: Retrieve a list of all available chat rooms.

   - **Precondition**: Use a valid JWT token.

     ```bash
     curl -X GET http://127.0.0.1:8080/api/rooms \
          -H "Authorization: Bearer $TOKEN"
     ```

   - **Expected Result**:

     You should receive a `200 OK` response with a JSON array of available rooms:

     ```json
     {
       "req_user_id": <your_user_id>,
       "rooms": [
         {
           "room_id": 1,
           "room_name": "testroom1",
           "user_id": <owner_user_id>
         }
       ]
     }
     ```

4. **Test the Add Member Endpoint (`POST /api/rooms/{room_id}/members`)**:

   - **Description**: Add the current user to a chat room by providing the room ID.

   - **Precondition**: Use a valid JWT token and ensure that the `room_id` you provide exists.

     ```bash
     curl -X POST http://127.0.0.1:8080/api/rooms/2/members \
          -H "Authorization: Bearer $TOKEN"
     ```

   - **Expected Result**:

     On success, you should receive a `200 OK` response confirming that the user has joined the room:

     ```json
     {
       "message": "User added to the room successfully"
     }
     ```

     If the specified `room_id` does not exist, you should receive a `404 Not Found` response.

5. **Test the Retrieve Room Members Endpoint (`GET /api/rooms/{room_id}/members`)**:

   - **Description**: Retrieve a list of all members in a specific chat room.

   - **Precondition**: Use a valid JWT token and provide a valid `room_id`.

     ```bash
     curl -X GET http://127.0.0.1:8080/api/rooms/1/members \
          -H "Authorization: Bearer $TOKEN"
     ```

   - **Expected Result**:

     On success, you should receive a `200 OK` response with a JSON array of members in the specified room:

     ```json
     [
       {
         "user_id": 1,
         "username": "testuser1"
       },
       {
         "user_id": 2,
         "username": "testuser2"
       }
     ]
     ```

     If the `room_id` does not exist, you should receive a `404 Not Found` response.

6. **Verify User-Room Relationship in the Database**:

   - **Description**: Check that the user is associated with the chat room in the `user_rooms` table.
   - **Command**:

     ```sql
     SELECT * FROM user_rooms WHERE user_id = <your_user_id> AND room_id = 1;
     ```

   - **Expected Result**: You should see an entry in the `user_rooms` table indicating the relationship between the user and the room.

---

### Steps to Test WebSocket Chat Functionality

#### Step 1: Create a Chat Room

1. **Add a Room**:

   Use the following `curl` command to create a new chat room. This room will be identified by a unique room ID.

   ```bash
   curl -X POST http://127.0.0.1:8080/api/rooms \
        -H "Authorization: Bearer $TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"room_name": "testroom1"}'
   ```

2. **View Room Info (to Get Room ID)**:

   Run this command to list all rooms and get the room ID for the newly created room.

   ```bash
   curl -X GET http://127.0.0.1:8080/api/rooms \
        -H "Authorization: Bearer $TOKEN"
   ```

   The response will include the room ID, which will be needed for the next steps.

#### Step 2: Add User to the Room

To allow a user to join a room via WebSocket, they must first be added as a member of that room. Use the room ID from the previous step.

```bash
curl -X POST http://127.0.0.1:8080/api/rooms/<room_id>/members \
     -H "Authorization: Bearer $TOKEN"
```

Replace `<room_id>` with the actual room ID obtained in Step 2.

#### Step 3: Connect to the Chat Room Using Websocat

1. **Install Websocat** (if not installed):

   ```bash
   cargo install websocat
   ```

2. **Join the WebSocket Room**:

   Use `websocat` to connect to the WebSocket endpoint of the specified chat room, including the JWT token in the `Authorization` header.

   ```bash
   websocat -H="Authorization: Bearer $TOKEN" ws://127.0.0.1:8080/ws/rooms/<room_id>
   ```

   Replace `<room_id>` with the room ID from Step 2.

#### Step 4: Send and Receive Messages

1. **Send a Message**:

   - After connecting to the WebSocket room using `websocat`, type messages directly into the terminal.
   - Type a message (e.g., `Hello, everyone!`) and press **Enter**. This message will be sent to the WebSocket server and should be broadcasted to all users connected to the same room.

   **Example**:

   ```plaintext
   Hello, everyone!
   ```

2. **View Received Messages**:

   - Any message sent by you or other users in the same chat room will appear in your terminal in real time.
   - For example, if another user in the room sends a message, it will be displayed in your `websocat` terminal as soon as the server broadcasts it.

   **Example Output**:

   ```plaintext
   User 2: Hello, everyone!
   ```

3. **Multiple Sessions for Testing**:

   - Open additional terminal windows or tabs and run the same `websocat` command to simulate multiple users in the chat room.
   - Each session should display any message sent by others, allowing you to test real-time broadcasting of messages.

4. **Message Flow Example**:

   - In terminal 1:

     ```plaintext
     Hello from User 1!
     ```

     Expected output in terminal 2:

     ```plaintext
     User 1: Hello from User 1!
     ```

   - In terminal 2:

     ```plaintext
     Hi User 1, this is User 2!
     ```

     Expected output in terminal 1:

     ```plaintext
     User 2: Hi User 1, this is User 2!
     ```

#### Step 5: User Join/Leave Notifications

1. **Join Notifications**:

   When a new `websocat` session connects to the room, all connected users should see a message like `"User <ID> has joined the room."`

2. **Leave Notifications**:

   When a `websocat` session disconnects (e.g., by pressing `Ctrl+C`), remaining users should see a message like `"User <ID> has left the room."`

#### Step 6: Verify User Presence

1. **Get User Presence in the Room**:

   Use the following command to retrieve the presence status of all users in a specific room. Replace `<room_id>` with the actual room ID obtained in Step 1.

   ```bash
   curl -X GET "http://127.0.0.1:8080/api/users/presence/<room_id>" \
        -H "Authorization: Bearer $TOKEN"
   ```

   The response will include a list of users in the room along with their presence status (`is_online: true` or `is_online: false`).

   **Example Response**:

   ```json
   [
     {"user_id": 1, "is_online": true},
     {"user_id": 2, "is_online": false}
   ]
   ```

   This output indicates that User 1 is currently online, while User 2 has disconnected.

2. **Test Real-Time Updates of Presence Status**:

   - **Step 1**: Connect multiple users to the room following **Step 3**.
   - **Step 2**: Run the presence check command above after each user joins to confirm they appear with `is_online: true`.
   - **Step 3**: Disconnect a user (e.g., by closing their `websocat` session) and run the command again to confirm their status changes to `is_online: false`.

   This step will validate that the server correctly tracks and updates presence information in real time as users join or leave.
