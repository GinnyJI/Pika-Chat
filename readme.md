## Reproducibility Guide

This guide will walk you through the steps to set up and run the server and client for the chatroom application.

### Server Setup

#### 1. Prepare the Environment

Ensure the following tools are installed:

- **Rust** and **Cargo**: Install them using the following command:
  
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

- SQLx CLI: Install it with SQLite support:

  ```bash
  cargo install sqlx-cli --no-default-features --features sqlite
  ```

---

#### 2. Set Up the Database

1. Navigate to the server directory and create the SQLite database file:

   ```bash
   cd pika-chatroom-server
   touch chat_app.db
   chmod 664 chat_app.db
   ```

2. Run the SQL migrations to set up the database schema:

   ```bash
   sqlx migrate run
   ```

   This will execute the migration scripts located in the `migrations/` directory.

---

#### 3. Build the Project

Compile the project and download all necessary dependencies:

```bash
cargo build
```

---

#### 4. Start the Server

Run the server with logging enabled to see detailed logs:

```bash
RUST_LOG=info cargo run
```

The server will run on [http://127.0.0.1:8080](http://127.0.0.1:8080/).
 You can access the Swagger UI for exploring and interacting with API endpoints at:
 http://127.0.0.1:8080/swagger-ui/.

---

### Client Setup

#### 1. Navigate to the frontend directory:

```bash
cd pika-chat-frontend
```

#### 2. Build the client application:

```bash
cargo build  # ⚡ Charging up your app with "Pika Power!"
```

#### 3. Start the client:

```bash
trunk serve  # 💬 Serving the chatroom faster than Pikachu's Quick Attack!
```

Visit [http://127.0.0.1:3000](http://127.0.0.1:3000/) to begin your chat journey!

