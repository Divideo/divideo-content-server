use std::path::PathBuf;

use rusqlite::Connection as SQLiteDatabase;

use divideo_models::{SearchQuery, VideoDescriptor};

pub trait Database
where
    Self: Sized,
{
    /// Opens a connection to a database file.
    fn open(path: impl Into<PathBuf>) -> Option<Self>;
    /// Gets a video from the database based on its ID.
    fn get_video(&self, id: u128) -> Option<VideoDescriptor>;
    /// Searches the database for videos matching the given query.
    fn search(&self, query: SearchQuery, start: usize, num: usize) -> Vec<VideoDescriptor>;
}

impl Database for SQLiteDatabase {
    fn open(path: impl Into<PathBuf>) -> Option<Self> {
        let db = Self::open(path.into()).ok()?;

        // Create the necessary tables if they don't exist.
        db.execute(
            "CREATE TABLE IF NOT EXISTS videos (
                id UUID PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                thumbnail TEXT,
            )",
            [],
        )
        .ok()?;

        Some(db)
    }

    fn get_video(&self, id: u128) -> Option<VideoDescriptor> {
        let mut query = self
            .prepare("SELECT id, title, description, thumbnail FROM videos WHERE id = (:id)")
            .ok()?;

        let video = query
            .query_map(&[(":id", &id.to_string())], |row| {
                let thumbnail_path: PathBuf = row.get::<usize, String>(3)?.into();
                let thumbnail = Reader::open(&thumbnail_path)
                    .unwrap_or(Err(rusqlite::Error::InvalidPath(thumbnail_path))?)
                    .decode()
                    .ok();

                Ok(VideoDescriptor {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    thumbnail,
                })
            })
            .ok()?
            .next()?
            .ok();

        // Clippy thinks we can avoid a let binding here, but we cannot.
        #[allow(clippy::let_and_return)]
        video
    }

    fn search(&self, query: SearchQuery, start: usize, num: usize) -> Vec<VideoDescriptor> {
        unimplemented!()
    }
}
