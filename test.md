### Steps to Test Registration, Login, and Logout

1. **Clean Up the Database**:
   - Open the SQLite CLI and connect to your database file:
     ```bash
     sqlite3 chat_app.db
     ```
   - Run this command to delete all users:
     ```sql
     DELETE FROM users;
     ```
   - Exit the SQLite CLI:
     ```sql
     .exit
     ```

2. **Test the Register Endpoint**:
   - Send a `POST` request to `http://127.0.0.1:8080/register` with a JSON payload containing a username and password.
   - Use `curl`:
     ```bash
     curl -X POST http://127.0.0.1:8080/register \
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
   - Send a `POST` request to `http://127.0.0.1:8080/login` with the same username and password.
   - Use `curl`:
     ```bash
     curl -X POST http://127.0.0.1:8080/login \
          -H "Content-Type: application/json" \
          -d '{"username": "testuser", "password": "password123"}'
     ```
   - Confirm that you receive a `200 OK` response with a token in the response body. Save this token for the logout test.

5. **Test the Logout Endpoint**:
   - Use the token obtained from the login response to send a `POST` request to `http://127.0.0.1:8080/logout`.
   - With `curl`, run:
     ```bash
     curl -X POST http://127.0.0.1:8080/logout \
          -H "Authorization: Bearer <your_token_here>"
     ```
   - Replace `<your_token_here>` with the actual token received from the login response.
   - Check that you receive a `200 OK` response with a message indicating a successful logout.

6. **Repeat as Needed**:
   - Repeat the cleanup, registration, login, and logout steps for different test cases or usernames.

---

### Steps to Test the Middleware

1. **Test with a Valid Token**:
   - Use `curl` to send a request with a valid token to a protected route:
     ```bash
     curl -H "Authorization: Bearer <your_valid_token>" http://127.0.0.1:8080/test-protected
     ```
   - Replace `<your_valid_token>` with a JWT that is accepted by your application.
   - **Expected Result**: You should receive a `200 OK` response with the message from the protected route.

2. **Test with an Invalid Token**:
   - Use `curl` to send a request with an invalid token:
     ```bash
     curl -H "Authorization: Bearer invalid_token" http://127.0.0.1:8080/test-protected
     ```
   - **Expected Result**: You should receive a `401 Unauthorized` response.

3. **Test Without a Token**:
   - Use `curl` to send a request without an `Authorization` header:
     ```bash
     curl http://127.0.0.1:8080/test-protected
     ```
   - **Expected Result**: You should receive a `401 Unauthorized` response.

---

Ensure your server is running with:
```bash
RUST_LOG=info cargo run
```

