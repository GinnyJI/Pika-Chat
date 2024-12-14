# Use the official Rust image as a build stage
FROM rust:1.78.0-buster as builder

# Set the working directory
WORKDIR /app

# Copy the project files into the container
COPY . .

# Install dependencies (no need for sqlx-cli or migration-related dependencies)
RUN apt-get update && apt-get install -y libsqlite3-dev
# Set runtime environment variables
ENV DATABASE_URL=sqlite:/app/chat_app.db
ENV SECRET_KEY=secret_key_for_jwt

# Build the project in release mode
RUN cargo build --release

# Prepare the runtime container
FROM debian:bookworm-slim

# Install the SQLite library (required by the application)
RUN apt-get update && apt-get install -y libsqlite3-0

# Copy the compiled application from the builder stage
COPY --from=builder /app/target/release/rust-chatroom-server /usr/local/bin/

# Copy the pre-migrated SQLite database from the local environment to the Docker image
COPY chat_app.db /app/chat_app.db

# Set the working directory for runtime
WORKDIR /app

# Run the application
CMD ["rust-chatroom-server"]
