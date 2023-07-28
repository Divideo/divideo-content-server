#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use uuid::Uuid;

/// Represents a video in the database, allowing the client to render video metadata,
/// then request the video to be streamed from the server.
pub struct VideoDescriptor {
    pub id: Uuid,
    pub title: String,
    // TODO: This should be a Markdown file
    pub description: String,
    // TODO: Comments
}

impl Display for VideoDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ID: {}
            Title: {}
            Description: {}",
            self.id, self.title, self.description,
        )
    }
}

/// Represents all data needed by the content/indexing server to serve search results to the client.
pub struct SearchQuery {
    tags: Tags,
    query: String,
}

/// Represents tags and their respective weights, used to rank search results.
pub struct Tags {
    pub tags: HashMap<String, f32>,
}
