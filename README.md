# ⚡️ pika-chat-frontend ⚡️

Welcome to **pika-chat-frontend**, where your chatroom sparks electrifying conversations! ⚡✨ Inspired by everyone's favorite Pokémon, Pikachu! 🟡

---

## Getting Started

Power up your chatroom with these simple steps:

```bash
cargo build  # ⚡ Charging up your app with "Pika Power!"
trunk serve  # 💬 Serving the chatroom faster than Pikachu's Quick Attack!
```

Visit [http://localhost:8080](http://localhost:8080) to start your chat journey! 

---

## Pikachu Reactions in the Chatroom (TODO)

- When a new user joins: ⚡✨ **"Pika Pi! Welcome to the chat!"**  
- When someone sends a great message: 🎉⚡ **"Pikachuuu~!"**  
- When someone leaves:  **"Pika-pika... Goodbye!"**

---

## Pikachu Meme of the Day

![Pikachu Meme](https://i.kym-cdn.com/photos/images/newsfeed/000/747/392/3a4.gif)

---

Now, let’s **spark** some fun and start chatting! ⚡💬

---

## Front End Structure Overview

Not really important... for anyone who is interested only

```bash
src/
├── components/                          # Reusable UI components for the frontend
│   ├── footer.rs                        # Footer component for the application layout
│   ├── form_input.rs                    # Form input component for user inputs
│   ├── header.rs                        # Header component for navigation and branding
│   ├── room_card.rs                     # Component to display a chat room summary
│   └── mod.rs                           # Module entry point for components
├── pages/                               # Page-specific components for routing
│   ├── dashboard.rs                     # Dashboard page for the logged-in user
│   ├── home.rs                          # Home page of the application
│   ├── login.rs                         # Login page for authentication
│   ├── register.rs                      # Register page for user sign-up
│   └── mod.rs                           # Module entry point for pages
├── services/                            # API service handlers for interacting with the backend
│   ├── auth.rs                          # Handles authentication API calls
│   ├── room.rs                          # Handles API calls related to chat room management
│   ├── utils.rs                         # Utility functions shared across services
│   └── mod.rs                           # Module entry point for services
├── static/                              # Static assets like images, fonts, and other media resources
├── styles/                              # CSS files for the application's styles
│   ├── base.css                         # Base styling for common elements
│   ├── dashboard.css                    # Styles specific to the dashboard page
│   ├── home.css                         # Styles for the home page
│   ├── login.css                        # Styles for the login page
│   ├── register.css                     # Styles for the register page
│   ├── output.css                       # Generated Tailwind CSS output
│   └── tailwind.css                     # Tailwind CSS customizations and imports
├── app.rs                               # Main application entry point for rendering and routing
├── routes.rs                            # Frontend routes configuration for navigation
├── index.html                           # HTML entry point for the application

```
