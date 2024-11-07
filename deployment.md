## Deployment Instructions for Rust Chatroom Server to Fly.io

This guide provides step-by-step instructions to set up a Linux environment, install necessary dependencies, prepare the database, and deploy a Rust application to Fly.io.

### Prerequisites

- **Multipass**: For creating and managing Ubuntu VMs on macOS or Windows.
- **Fly.io Account**: Create an account on [Fly.io](https://fly.io) if you haven’t already.

---

### 1. Set Up a Linux Environment with Multipass

Using Multipass allows for a consistent Linux environment to build and deploy your Rust project, avoiding platform-specific issues that can arise on macOS or Windows.

1. **Install Multipass**: Download and install [Multipass](https://multipass.run/).

2. **Create a New Ubuntu VM with Sufficient Disk Space**:
   - Launch an Ubuntu instance with at least 10 GB of disk space and 2 GB of memory:

     ```bash
     multipass launch --name ubuntu-rust --disk 10G --mem 2G
     ```

3. **Access the VM Shell**:
   - Connect to your new VM:

     ```bash
     multipass shell ubuntu-rust
     ```

---

### 2. Install Rust and Project Dependencies

1. **Install Rust**:
   - Download and install Rust using the following command:

     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     source $HOME/.cargo/env
     ```

2. **Install Build Essentials and SQLite Libraries**:
   - Some dependencies require a C compiler and SQLite development libraries. Install them using:

     ```bash
     sudo apt update
     sudo apt install -y build-essential libsqlite3-dev
     ```

3. **Install SQLx CLI for Database Migrations**:
   - Use `cargo` to install `sqlx-cli` with SQLite support:

     ```bash
     cargo install sqlx-cli --no-default-features --features sqlite
     ```

---

### 3. Clone and Prepare the Rust Project

1. **Clone or Transfer the Project**:
   
   - Clone the repository to your VM or transfer your project files:
   
     ```bash
     git clone https://github.com/yourusername/rust-chatroom-server.git
     cd rust-chatroom-server
     ```
   
3. **Remove Hidden Files in the Migrations Folder**:
   - Sometimes, hidden files can cause SQLx migration errors. Check for any unexpected files in the `migrations` directory:

     ```bash
     ls -a migrations
     ```

   - Delete any unnecessary or hidden files (like files starting with `._`):

     ```bash
     rm migrations/._0001_create_users.sql
     ```

---

### 4. Set Up the Database Locally

1. **Create the SQLite Database File**:
   - Create the SQLite database file with the correct permissions:

     ```bash
     touch chat_app.db
     chmod 664 chat_app.db
     ```

2. **Run Migrations**:
   - Use SQLx to run the database migrations. This will initialize the database schema in `chat_app.db`:

     ```bash
     sqlx migrate run --source ./migrations
     ```

   > **Note**: This step creates and configures the database prior to deployment, meaning that Fly.io will use the pre-configured database file.

---

### 5. Install Flyctl and Deploy to Fly.io

1. **Install Flyctl**:
   - Download and install Fly.io’s command-line tool, `flyctl`:

     ```bash
     curl -L https://fly.io/install.sh | sh
     ```

2. **Add Flyctl to PATH**:
   - Add `flyctl` to your PATH by editing `.bashrc`:

     ```bash
     nano ~/.bashrc
     ```

   - Add the following lines at the end of the file:

     ```bash
     export FLYCTL_INSTALL="$HOME/.fly"
     export PATH="$FLYCTL_INSTALL/bin:$PATH"
     ```

   - Save the file, then reload `.bashrc` to apply the changes:

     ```bash
     source ~/.bashrc
     ```

3. **Log in to Fly.io**:
   - Authenticate with Fly.io:

     ```bash
     flyctl auth login
     ```

4. **Initialize the Fly.io Application**:
   - If this is your first time deploying, initialize the app in the project directory:

     ```bash
     flyctl launch
     ```

   - Follow the prompts to set up your Fly.io app, including setting the app name, region, and builder type.

5. **Update Configuration in fly.toml**
   - Ensure your `fly.toml` configuration matches the following to map port `80` correctly:

     ```toml
     [[services]]
       internal_port = 80
       protocol = "tcp"

       [[services.ports]]
         handlers = ["http"]
         port = 80

       [[services.ports]]
         handlers = ["tls", "http"]
         port = 443
     ```

6. **Deploy to Fly.io**:
   - Deploy the application from the Ubuntu VM:

     ```bash
     flyctl deploy
     ```

   - Monitor the logs to ensure the deployment is successful.

---

### 6. Verify the Deployment

1. **Access the Application**:
   - Fly.io provides a URL for your application, such as `https://rust-chatroom-server.fly.dev`. Open this URL in your browser or use `curl` to test:

     ```bash
     curl https://rust-chatroom-server.fly.dev
     ```

2. **Test API Endpoints**:
   - Use `curl` or Postman to test various endpoints, like `/api/register`:

     ```bash
     curl -X POST https://rust-chatroom-server.fly.dev/api/register \
          -H "Content-Type: application/json" \
          -d '{"username": "testuser1", "password": "password123"}'
     ```

3. **Check Logs**:
   - If you need to troubleshoot or monitor the app, view logs with:

     ```bash
     flyctl logs
     ```

---

### Additional Notes

- **Database Configuration**: If you use sensitive environment variables like `DATABASE_URL`, it’s recommended to set them as secrets in Fly.io:

  ```bash
  flyctl secrets set DATABASE_URL=sqlite:/app/chat_app.db SECRET_KEY=your_secret_key
  ```

- **Removing Hidden Migration Files**: MacOS sometimes generates hidden files in directories (e.g., `._filename`), which can cause issues in deployments. Regularly check and clean the `migrations` directory for any such files.
