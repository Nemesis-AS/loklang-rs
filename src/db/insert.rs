// use diesel::upsert::excluded;
use diesel::RunQueryDsl; // ExpressionMethods

use crate::db::models::{AlbumInsertable, ArtistInsertable, SongInsertable};
use crate::db::types::DbPool;
use crate::metadata::AudioMetadata;

pub fn insert_metadata(pool: &DbPool, data: Vec<AudioMetadata>) {
    // use crate::db::schema::songs::dsl::*;

    // let songs_meta: Vec<SongInsertable> = data
    //     .into_iter()
    //     .map(|meta| SongInsertable {
    //         id: meta.id,
    //         title: meta.title,
    //         duration: Some(meta.duration as i32),
    //         file_path: meta.filepath,
    //         album_id: None,
    //         cover_image: None,
    //     })
    //     .collect();

    let mut conn = pool.get().unwrap();

    for item in data {
        let insertable: SongInsertable = SongInsertable {
            id: item.id,
            title: item.title,
            duration: Some(item.duration as i32),
            file_path: item.filepath,
            album_id: None,
            cover_image: None,
        };

        let artists = item.artists;
        let album_title = item.album;

        // @todo! Case insensitive search in DB for artists

        for artist in artists {
            use crate::db::schema::artists::dsl::*;

            let artist_id = format!("{:x}", md5::compute(&artist));
            let art: ArtistInsertable = ArtistInsertable {
                id: artist_id,
                name: artist.clone(),
                sort_name: artist.clone(),
            };

            diesel::insert_into(artists)
                .values(art)
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .expect("An error occurred while inserting artist into db!");
        }

        let album_id: String = format!("{:x}", md5::compute(&album_title));
        let album: AlbumInsertable = AlbumInsertable {
            id: album_id,
            title: album_title.clone(),
            release_date: None,
            cover_image: None,
        };

        {
            use crate::db::schema::albums::dsl::*;

            diesel::insert_into(albums)
                .values(album)
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .expect("An error occurred while inserting album into db!");
        }

        {
            use crate::db::schema::songs::dsl::*;

            diesel::insert_into(songs)
                .values(&insertable)
                .on_conflict_do_nothing()
                .execute(&mut conn)
                .expect("An error occurred while inserting songs!");
        }
    }

    // @todo! Make this upsert work
    // diesel::insert_into(songs)
    //     .values(&songs_meta)
    //     .on_conflict(id)
    //     .do_update()
    //     .set((
    //         id.eq(excluded(id)),
    //         title.eq(excluded(title)),
    //         duration.eq(excluded(duration)),
    //         file_path.eq(excluded(file_path)),
    //         album_id.eq(excluded(album_id)),
    //         cover_image.eq(excluded(cover_image)),
    //         created_at.eq(excluded(created_at))
    //     ))
    //     .execute(&mut conn)
    //     .expect("An error occurred while inserting songs!");

    // for song in songs_meta {
    //     diesel::insert_into(songs::table)
    //         .values(&song)
    //         .on_conflict_do_nothing()
    //         .execute(&mut conn)
    //         .expect("An error occurred while inserting songs!");
    // }
}
