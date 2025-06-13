// @generated automatically by Diesel CLI.

diesel::table! {
    album_artists (album_id, artist_id) {
        album_id -> Text,
        artist_id -> Text,
    }
}

diesel::table! {
    albums (id) {
        id -> Text,
        title -> Text,
        release_date -> Nullable<Text>,
        cover_image -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    artists (id) {
        id -> Text,
        name -> Text,
        sort_name -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    playlist_songs (playlist_id, song_id) {
        playlist_id -> Text,
        song_id -> Text,
        position -> Integer,
    }
}

diesel::table! {
    playlists (id) {
        id -> Text,
        name -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    song_artists (song_id, artist_id) {
        song_id -> Text,
        artist_id -> Text,
        role -> Nullable<Text>,
    }
}

diesel::table! {
    songs (id) {
        id -> Text,
        title -> Text,
        file_path -> Text,
        duration -> Nullable<Integer>,
        album_id -> Nullable<Text>,
        cover_image -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::joinable!(album_artists -> albums (album_id));
diesel::joinable!(album_artists -> artists (artist_id));
diesel::joinable!(playlist_songs -> playlists (playlist_id));
diesel::joinable!(playlist_songs -> songs (song_id));
diesel::joinable!(song_artists -> artists (artist_id));
diesel::joinable!(song_artists -> songs (song_id));
diesel::joinable!(songs -> albums (album_id));

diesel::allow_tables_to_appear_in_same_query!(
    album_artists,
    albums,
    artists,
    playlist_songs,
    playlists,
    song_artists,
    songs,
);
