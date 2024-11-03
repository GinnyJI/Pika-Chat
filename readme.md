## Structure

```graphql
src/
├── config/          # Configuration-related files, e.g., state management or app settings
│   ├── mod.rs       # Module entry point for config
│   └── state.rs     # State management file (e.g., app state)
├── models/          # Database models and data structures
│   ├── mod.rs       # Module entry point for models
│   ├── claim.rs     # JWT claims structure
│   └── user.rs      # User data model
├── routes/          # Route handlers and logic
│   ├── auth.rs      # Handlers for authentication routes
│   ├── test_routes.rs # Handlers for test routes (e.g., for testing middleware)
│   └── mod.rs       # Module entry point to export all routes
├── middleware/      # Middleware, e.g., JWT authentication
│   ├── auth_middleware.rs # Middleware logic for authentication
│   └── mod.rs       # Module entry point for middleware
├── main.rs          # Main application entry point

```
