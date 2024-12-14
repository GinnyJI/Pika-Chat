CREATE TABLE rooms (
    room_id INTEGER PRIMARY KEY AUTOINCREMENT,
    room_name TEXT UNIQUE NOT NULL,
    user_id INTEGER NOT NULL, -- The ID of the user who created the room
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id) -- Foreign key to link to the users table
);


CREATE TABLE user_rooms (
    user_id INTEGER,
    room_id INTEGER,
    joined_at TEXT DEFAULT (datetime('now')),
    PRIMARY KEY (user_id, room_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (room_id) REFERENCES rooms(room_id)
);
