## Steps to Run the Project

1. **Build the Project**:
   
   Compile the project and download all dependencies:
   ```bash
   cargo build
   ```
   
2. **Run Database Migrations**:
   
   Ensure you are in the project root directory and execute the SQL migrations:
   ```bash
   sqlx migrate run
   ```

   This step will create or update the database schema according to the migration scripts in the `migrations/` directory.
   
3. **Start the Server**:
   
   Run the server with logging enabled:
   ```bash
   RUST_LOG=info cargo run
   ```
   
   The server should start and listen on `http://127.0.0.1:8080` by default. Ensure that all logs are printed to verify the server's status and any potential issues.

### Additional Notes
- **Environment Setup**:
  
  Ensure you have a `.env` file with the `DATABASE_URL` set to your SQLite database, such as:
  ```env
  DATABASE_URL=sqlite:chat_app.db
  ```
  
- **Initial Setup**:
  
  If running for the first time, confirm that you have `sqlx-cli` installed:
  ```bash
  cargo install sqlx-cli --no-default-features --features sqlite
  ```
  
- **Verifying the Server**:
  
  You can use `curl` or a browser to check the server status:
  ```bash
  curl http://127.0.0.1:8080
  ```

These steps will get your project up and running smoothly for testing and development.


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
