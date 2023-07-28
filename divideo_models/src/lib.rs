#![allow(dead_code)]

use std::collections::HashMap;

use image::DynamicImage;
use uuid::Uuid;

/// Represents a video in the database, allowing the client to render video metadata,
/// then request the video to be streamed from the server.
pub struct VideoDescriptor {
    pub id: Uuid,
    pub title: String,
    // TODO: This should be a Markdown file
    pub description: String,
    pub thumbnail: Option<DynamicImage>,
    // TODO: Comments
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
