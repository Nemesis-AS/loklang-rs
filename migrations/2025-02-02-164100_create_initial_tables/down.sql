-- Disable foreign key checks before dropping tables
PRAGMA foreign_keys = OFF;

-- Drop Many-to-Many Relationships First
DROP TABLE IF EXISTS playlist_songs;
DROP TABLE IF EXISTS song_artists;
DROP TABLE IF EXISTS album_artists;

-- Drop Core Tables
DROP TABLE IF EXISTS playlists;
DROP TABLE IF EXISTS songs;
DROP TABLE IF EXISTS albums;
DROP TABLE IF EXISTS artists;

-- Re-enable foreign key checks
PRAGMA foreign_keys = ON;