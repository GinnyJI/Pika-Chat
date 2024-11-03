## Structure

```graphql
src/
├── config/          # Config-related files, if needed
├── models/          # Database models
├── routes/          # Route handlers and logic
│   ├── auth.rs
│   ├── mod.rs       # Module to export all routes
├── middleware/      # Middleware, e.g., JWT auth
main.rs
```
