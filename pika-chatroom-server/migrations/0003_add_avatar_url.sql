-- Migration script to add avatar_url column to the users table
ALTER TABLE users ADD COLUMN avatar_url TEXT DEFAULT NULL;
