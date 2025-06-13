use crate::db::{
    models::{Album, Artist, Song, SongRes},
    schema::{albums, artists, song_artists},
    types::DbError,
};
use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};

pub fn join_song_info(
    songs_vec: Vec<Song>,
    conn: &mut diesel::r2d2::PooledConnection<
        diesel::r2d2::ConnectionManager<diesel::SqliteConnection>,
    >,
) -> Result<Vec<SongRes>, DbError> {
    let song_res: Vec<SongRes> = songs_vec
        .iter()
        .map(|song| -> SongRes {
            let album_data: Option<Album> = song.album_id.as_ref().and_then(|album_id_val| {
                albums::table
                    .find(album_id_val)
                    .select(Album::as_select())
                    .first(conn)
                    .optional()
                    .expect("An error occurred while fetching album")
            });

            let song_artist_vec: Vec<Artist> = song_artists::table
                .inner_join(artists::table.on(song_artists::artist_id.eq(artists::id)))
                .filter(song_artists::song_id.eq(&song.id))
                .select(Artist::as_select())
                .load(conn)
                .unwrap();

            SongRes {
                id: song.id.clone(),
                title: song.title.clone(),
                album_id: song.album_id.clone(),
                cover_image: song.cover_image.clone(),
                duration: song.duration,
                file_path: song.file_path.clone(),
                created_at: song.created_at,
                album: album_data,
                artists: Some(song_artist_vec),
            }
        })
        .collect();

    Ok(song_res)
}

pub fn join_song_info_single(
    song_data: Song,
    conn: &mut diesel::r2d2::PooledConnection<
        diesel::r2d2::ConnectionManager<diesel::SqliteConnection>,
    >,
) -> Result<SongRes, DbError> {
    let album_data: Option<Album> = song_data.album_id.as_ref().and_then(|album_id_val| {
        albums::table
            .find(album_id_val)
            .select(Album::as_select())
            .first(conn)
            .optional()
            .expect("An error occurred while fetching album")
    });

    let song_artist_vec: Vec<Artist> = song_artists::table
        .inner_join(artists::table.on(song_artists::artist_id.eq(artists::id)))
        .filter(song_artists::song_id.eq(&song_data.id))
        .select(Artist::as_select())
        .load(conn)
        .unwrap();

    Ok(SongRes {
        id: song_data.id.clone(),
        title: song_data.title.clone(),
        album_id: song_data.album_id.clone(),
        cover_image: song_data.cover_image.clone(),
        duration: song_data.duration,
        file_path: song_data.file_path.clone(),
        created_at: song_data.created_at,
        album: album_data,
        artists: Some(song_artist_vec),
    })

    // Ok(song_res)
}
