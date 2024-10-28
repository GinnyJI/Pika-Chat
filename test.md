Here’s the updated test procedure, now including the steps to test the logout endpoint.

---

### Steps to Test Registration, Login, and Logout

```bash
RUST_LOG=info cargo run
```

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
   - With `curl`, you can run:
     ```bash
     curl -X POST http://127.0.0.1:8080/logout \
          -H "Authorization: Bearer <your_token_here>"
     ```
   - Replace `<your_token_here>` with the actual token received from the login response.
   - Check that you receive a `200 OK` response with a message indicating a successful logout.

6. **Repeat as Needed**:
   - Repeat the cleanup, registration, login, and logout steps for different test cases or usernames.
