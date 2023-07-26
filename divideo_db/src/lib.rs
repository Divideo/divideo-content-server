use std::path::PathBuf;

use divideo_models::{SearchQuery, VideoDescriptor};

pub trait Database
where
    Self: Sized,
{
    fn open(path: impl Into<PathBuf>) -> Option<Self>;
    fn search(&mut self, query: SearchQuery, start: usize, num: usize) -> Vec<VideoDescriptor>;
}
