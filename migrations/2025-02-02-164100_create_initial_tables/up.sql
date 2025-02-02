PRAGMA foreign_keys = ON;

-- Artists Table
CREATE TABLE artists (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    sort_name TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Albums Table (Releases in MusicBrainz)
CREATE TABLE albums (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    release_date TEXT, -- YYYY-MM-DD
    cover_image TEXT, -- File path or URL
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Songs Table (Recordings in MusicBrainz)
CREATE TABLE songs (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    file_path TEXT NOT NULL, -- Path to the song file
    duration INTEGER, -- Duration in seconds
    album_id TEXT NULL, -- Null for singles
    cover_image TEXT NULL, -- Override album cover if needed
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (album_id) REFERENCES albums(id) ON DELETE SET NULL
);

-- Many-to-Many Table for Songs & Artists
CREATE TABLE song_artists (
    song_id TEXT NOT NULL,
    artist_id TEXT NOT NULL,
    role TEXT DEFAULT 'performer', -- E.g., 'composer', 'producer'
    PRIMARY KEY (song_id, artist_id),
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE,
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE
);

-- Many-to-Many Table for Albums & Artists
CREATE TABLE album_artists (
    album_id TEXT NOT NULL,
    artist_id TEXT NOT NULL,
    PRIMARY KEY (album_id, artist_id),
    FOREIGN KEY (album_id) REFERENCES albums(id) ON DELETE CASCADE,
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE
);

-- Playlists Table
CREATE TABLE playlists (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Many-to-Many Table for Playlists & Songs
CREATE TABLE playlist_songs (
    playlist_id TEXT NOT NULL,
    song_id TEXT NOT NULL,
    position INTEGER NOT NULL, -- Track order in the playlist
    PRIMARY KEY (playlist_id, song_id),
    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE
);
