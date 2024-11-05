## Steps to Run the Project

1. **Prepare the Environment**:
   - Ensure that you have `Rust`, `Cargo`, and `SQLx CLI` installed. If not, install them using:

     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     cargo install sqlx-cli --no-default-features --features sqlite
     ```

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

### Additional Notes:

- **Reset Database for Development**:
  - To reset the database, delete the `chat_app.db` file and re-run migrations:

    ```bash
    rm chat_app.db
    sqlx migrate run
    ```

## Steps to Test APIs

1. **Clean Up the Database**:
   - Open the SQLite CLI and connect to your database file:

     ```bash
     sqlite3 chat_app.db
     ```

   - Run these commands to delete all users and reset the `AUTOINCREMENT` counter:

     ```sql
     DELETE FROM users;
     DELETE FROM sqlite_sequence WHERE name = 'users';
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
          -d '{"username": "testuser", "password": "password123"}'
     ```

   - Verify that you receive a `201 Created` response, indicating the user was created successfully.

3. **Verify User Creation in Database**:

   - Reopen the SQLite CLI and check that the user was created:

     ```sql
     SELECT * FROM users WHERE username = 'testuser';
     ```

   - Ensure that `testuser` appears in the results.

4. **Test the Login Endpoint**:

   - Send a `POST` request to `http://127.0.0.1:8080/api/login` with the same username and password.
   - Use `curl`:

     ```bash
     curl -X POST http://127.0.0.1:8080/api/login \
          -H "Content-Type: application/json" \
          -d '{"username": "testuser", "password": "password123"}'
     ```

   - Confirm that you receive a `200 OK` response with a token in the response body. Save this token for the logout test.

5. **Test the Logout Endpoint**:

   - Use the token obtained from the login response to send a `POST` request to `http://127.0.0.1:8080/api/logout`.
   - With `curl`, run:

     ```bash
     curl -X POST http://127.0.0.1:8080/api/logout \
          -H "Authorization: Bearer <your_token_here>"
     ```

   - Replace `<your_token_here>` with the actual token received from the login response.
   - Check that you receive a `200 OK` response with a message indicating a successful logout.

---

### Steps to Test the Middleware

1. **Test with a Valid Token**:
   - Use `curl` to send a request with a valid token to a protected route:

     ```bash
     curl -H "Authorization: Bearer <your_valid_token>" http://127.0.0.1:8080/api/test-protected
     ```

   - Replace `<your_valid_token>` with a JWT that is accepted by your application.
   - **Expected Result**: You should receive a `200 OK` response with the message from the protected route.

2. **Test with an Invalid Token**:

   - Use `curl` to send a request with an invalid token:

     ```bash
     curl -H "Authorization: Bearer invalid_token" http://127.0.0.1:8080/api/test-protected
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
        -H "Authorization: Bearer <your_token_here>" \
        -H "Content-Type: application/json" \
        -d '{"room_name": "testroom"}'
   ```

   - **Expected Result**:
     - On success, you should receive a `201 Created` response with a JSON body containing the `room_id` of the newly created room:
       ```json
       { "room_id": "int" }
       ```
     - If the room name already exists, you should receive a `400 Bad Request` response indicating a duplicate room name.

2. **Verify Room Creation in the Database**:

   - **Description**: Check that the room was correctly created by querying the database.
   - **Command**:

     ```sql
     SELECT * FROM rooms WHERE room_name = 'testroom';
     ```

   - **Expected Result**: You should see an entry in the `rooms` table with the name `testroom` and an associated `room_id`.

3. **Test the Retrieve Rooms Endpoint (`GET /api/rooms`)**:

   - **Description**: Retrieve a list of all available chat rooms.
   - **Precondition**: Use a valid JWT token.

   ```bash
   curl -X GET http://127.0.0.1:8080/api/rooms \
        -H "Authorization: Bearer <your_token_here>"
   ```

   - **Expected Result**:
     - You should receive a `200 OK` response with a JSON array of available rooms:
       ```json
       [
         { "room_id": 1, "room_name": "testroom" },
         { "room_id": 2, "room_name": "anotherroom" }
       ]
       ```

4. **Test the Join Room Endpoint (`POST /api/rooms/{room_id}/join`)**:

   - **Description**: Join a specific chat room by providing the room ID.
   - **Precondition**: Use a valid JWT token and ensure that the `room_id` you provide exists.

   ```bash
   curl -X POST http://127.0.0.1:8080/api/rooms/1/join \
        -H "Authorization: Bearer <your_token_here>"
   ```

   - **Expected Result**:
     - On success, you should receive a `200 OK` response confirming that the user has joined the room.
     - If the specified `room_id` does not exist, you should receive a `404 Not Found` response.

5. **Verify User-Room Relationship in the Database**:

   - **Description**: Check that the user is associated with the chat room in the `user_rooms` table.
   - **Command**:

     ```sql
     SELECT * FROM user_rooms WHERE user_id = <your_user_id> AND room_id = 1;
     ```

   - **Expected Result**: You should see an entry in the `user_rooms` table indicating the relationship between the user and the room.

---

