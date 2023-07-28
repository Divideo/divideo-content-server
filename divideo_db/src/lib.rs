#![allow(unused)]

use std::path::PathBuf;

use anyhow::Result;
use rusqlite::Connection as SQLiteDatabase;
use uuid::Uuid;

use divideo_models::{SearchQuery, VideoDescriptor};

pub trait Database
where
    Self: Sized,
{
    /// Opens a connection to a database file.
    fn open_database(path: impl Into<PathBuf>) -> Result<Self>;
    /// Uploads a video into the database.
    fn upload_video(&self, id: Uuid, title: &str, description: &str) -> Result<()>;
    /// Gets a video from the database based on its ID.
    fn get_video(&self, id: Uuid) -> Result<VideoDescriptor>;
    /// Searches the database for videos matching the given query.
    fn search(&self, query: SearchQuery, start: usize, num: usize) -> Result<Vec<VideoDescriptor>>;
}

impl Database for SQLiteDatabase {
    fn open_database(path: impl Into<PathBuf>) -> Result<Self> {
        let db = Self::open(path.into())?;

        // Create the necessary tables if they don't exist.
        db.execute(
            "CREATE TABLE IF NOT EXISTS videos (
                id UUID PRIMARY KEY NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
            )",
            [],
        )?;

        Ok(db)
    }

    fn upload_video(&self, id: Uuid, title: &str, description: &str) -> Result<()> {
        let mut query = self.prepare(
            "INSERT INTO videos (id, title, description) VALUES (:id, :title, :description)",
        )?;

        let id_string: &str = &id.to_string();
        query.execute(&[
            (":id", id_string),
            (":title", title),
            (":description", description),
        ])?;

        Ok(())
    }

    fn get_video(&self, id: Uuid) -> Result<VideoDescriptor> {
        let mut query = self.prepare("SELECT id, title, description, FROM videos WHERE id == ?")?;

        let mut rows = query.query([&id.to_string()])?;

        // ? How do I error handle this?
        let row = rows.next()?.unwrap();

        // The ID must be converted from a string to a UUID.
        let id_string: String = row.get(0)?;
        let id = Uuid::try_parse(&id_string)?;

        Ok(VideoDescriptor {
            id,
            title: row.get(1)?,
            description: row.get(2)?,
        })
    }

    fn search(&self, query: SearchQuery, start: usize, num: usize) -> Result<Vec<VideoDescriptor>> {
        unimplemented!()
    }
}
