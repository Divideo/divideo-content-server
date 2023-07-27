use std::collections::HashMap;

type Id = u128;

/// Represents a video in the database, allowing the client to render video metadata,
/// then request the video to be streamed from the server.
pub struct VideoDescriptor {
    pub id: Id,
    pub title: String,
    pub description: String,
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
